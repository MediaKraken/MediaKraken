//! Live HLS transcode sessions.
//!
//! A `HlsStart` message spawns one ffmpeg process per ABR rendition, each
//! writing a live-style HLS playlist under `HLS_ROOT/<guid>/`. The web app
//! serves those files to the browser through a session-authenticated route;
//! the playlists are not cached, so the growing stream is picked up as soon
//! as segments appear. A `HlsStop` message kills the ffmpeg children and
//! deletes the segment directory.
//!
//! All segments live on the shared `mkstack-hls-claim` volume: mktranscode
//! mounts it read-write, the web app mounts it read-only. Segments are a
//! working set, not a store: they are deleted on stop and swept by age when
//! no stop ever arrived (page crash, pod restart).

use mk_lib_common::mk_lib_common_ffmpeg;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::process::{Child, Command};
use uuid::Uuid;

/// Mount point of the shared HLS working volume (see k8s chart).
pub const HLS_ROOT: &str = "/mediakraken/hls";
/// The scanner stores `mm_media_path` with a connection header —
/// `smb://<ip>/<share>/...` or `nfs://<ip>/<path>/...` (see
/// mkmediascanner::mk_media_path_prefix) — and ffmpeg opens those URLs
/// directly, the same way the ffprobe pass does. Local absolute paths are
/// accepted too, for local-library deployments.
pub fn is_supported_media_path(media_path: &str) -> bool {
    media_path.starts_with("smb://")
        || media_path.starts_with("nfs://")
        || media_path.starts_with('/')
}
const HLS_SEGMENT_SECONDS: i32 = 6;
const KEYFRAME_INTERVAL_FRAMES: i32 = 120;
const KEYFRAME_MIN_INTERVAL_FRAMES: i32 = 60;
/// Session directories not written to for this long are deleted.
const HLS_SWEEP_AGE: Duration = Duration::from_hours(24);
/// Minimum time between directory sweeps.
const HLS_SWEEP_INTERVAL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Copy)]
struct RenditionPlan {
    /// Target output height in pixels; 0 = native (no scaling).
    height: u32,
    /// Peak bitrate declared in the master playlist, bits/s.
    bandwidth: u64,
}

impl RenditionPlan {
    /// File label used for the variant playlist and segment pattern.
    fn label(self) -> String {
        if self.height > 0 {
            self.height.to_string()
        } else {
            "src".to_string()
        }
    }
}

/// One ffmpeg child per ABR rendition for a single media item.
struct HlsSession {
    children: Vec<Child>,
}

pub struct HlsManager {
    sessions: HashMap<Uuid, HlsSession>,
    last_sweep: Instant,
}

fn plan_renditions(source_height: Option<u32>) -> Vec<RenditionPlan> {
    // ABR ladder 720p + 1080p; sources at or below 720p get a single
    // native-resolution rendition (scaling them up would only cost CPU and
    // disk without any visual benefit).
    match source_height {
        Some(height) if height > 1080 => vec![
            RenditionPlan {
                height: 1080,
                bandwidth: 5_128_000,
            },
            RenditionPlan {
                height: 720,
                bandwidth: 2_628_000,
            },
        ],
        Some(height) if height > 720 => vec![
            RenditionPlan {
                height,
                bandwidth: 5_128_000,
            },
            RenditionPlan {
                height: 720,
                bandwidth: 2_628_000,
            },
        ],
        _ => vec![RenditionPlan {
            height: 0,
            bandwidth: 2_628_000,
        }],
    }
}

fn ffmpeg_args(media_path: &str, dir: &Path, rendition: &RenditionPlan) -> Vec<String> {
    let mut args = vec![
        "-hide_banner".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
        "-nostdin".to_string(),
        "-y".to_string(),
        "-i".to_string(),
        media_path.to_string(),
        // Best video stream plus the best audio stream when present; the
        // trailing `?` keeps audio optional for silent sources.
        "-map".to_string(),
        "0:v:0".to_string(),
        "-map".to_string(),
        "0:a?".to_string(),
    ];
    if rendition.height > 0 {
        // `min(H,ih)` never up-scales: sources shorter than the target keep
        // their native height. `-2` keeps the width even.
        args.push("-vf".to_string());
        args.push(format!("scale=-2:min({}\\,ih)", rendition.height));
    }
    args.extend([
        "-c:v".to_string(),
        "libx264".to_string(),
        "-preset".to_string(),
        "veryfast".to_string(),
        "-pix_fmt".to_string(),
        "yuv420p".to_string(),
        // Keyframes at least every ~5s so the 6s segments can stay
        // independently decodable.
        "-g".to_string(),
        KEYFRAME_INTERVAL_FRAMES.to_string(),
        "-keyint_min".to_string(),
        KEYFRAME_MIN_INTERVAL_FRAMES.to_string(),
    ]);
    let label = rendition.label();
    args.extend([
        "-b:v".to_string(),
        format!("{}k", (rendition.bandwidth - 128_000) as u32 / 1000),
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        "-ar".to_string(),
        "48000".to_string(),
        "-f".to_string(),
        "hls".to_string(),
        "-hls_time".to_string(),
        HLS_SEGMENT_SECONDS.to_string(),
        // Keep every segment ever written so the player can seek back over
        // the whole transcoded range.
        "-hls_list_size".to_string(),
        "0".to_string(),
        "-hls_flags".to_string(),
        "independent_segments".to_string(),
        "-hls_segment_filename".to_string(),
        dir.join(format!("{label}_seg_%05d.ts"))
            .to_string_lossy()
            .into_owned(),
        dir.join(format!("{label}.m3u8"))
            .to_string_lossy()
            .into_owned(),
    ]);
    args
}

/// Best-effort source dimensions from ffprobe; `None` when the probe fails
/// or the source has no video stream.
async fn probe_source_dimensions(media_path: &str) -> Option<(u32, u32)> {
    let probe: Value = match mk_lib_common_ffmpeg::mk_common_ffmpeg_get_info(media_path).await {
        Ok(value) => value,
        Err(error) => {
            eprintln!("mktranscode: hls ffprobe failed for '{media_path}' ({error})");
            return None;
        }
    };
    let streams = probe.get("streams")?.as_array()?;
    for stream in streams {
        if stream.get("codec_type").and_then(Value::as_str) == Some("video")
            && let Some(width) = stream.get("width").and_then(Value::as_u64)
            && let Some(height) = stream.get("height").and_then(Value::as_u64)
            && width > 0
            && height > 0
        {
            return Some((width.try_into().ok()?, height.try_into().ok()?));
        }
    }
    None
}

/// Builds the master playlist body for the given renditions.
fn master_playlist_body(renditions: &[RenditionPlan], source: Option<(u32, u32)>) -> String {
    let mut body = String::from("#EXTM3U\n#EXT-X-INDEPENDENT-SEGMENTS\n");
    for rendition in renditions {
        let mut inf = format!("#EXT-X-STREAM-INF:BANDWIDTH={}", rendition.bandwidth);
        if let Some((src_width, src_height)) = source
            && rendition.height > 0
        {
            let out_height = rendition.height.min(src_height);
            let out_width =
                (src_width as u64).saturating_mul(out_height as u64) / (src_height.max(1) as u64);
            // Even the width to match the encoder output.
            let out_width = out_width & !1;
            inf.push_str(&format!(",RESOLUTION={out_width}x{out_height}"));
        }
        inf.push('\n');
        inf.push_str(&format!("{}.m3u8\n", rendition.label()));
        body.push_str(&inf);
    }
    body
}

/// Age of a session directory, or `None` when it cannot be determined.
async fn directory_age(dir: &Path) -> Option<Duration> {
    let modified = tokio::fs::metadata(dir).await.ok()?.modified().ok()?;
    let now = std::time::SystemTime::now();
    now.duration_since(modified).ok()
}

impl HlsManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            last_sweep: Instant::now(),
        }
    }

    fn kill_session(session: &mut HlsSession) {
        for child in session.children.iter_mut() {
            // Fire and forget: kill is synchronous on the OS side and the
            // child will be reaped by a later `reap()`.
            let _ = child.start_kill();
        }
    }

    /// Removes the session and its segment directory. Best effort.
    async fn remove_session(&mut self, guid: &Uuid) {
        if let Some(mut session) = self.sessions.remove(guid) {
            Self::kill_session(&mut session);
        }
        let dir = Path::new(HLS_ROOT).join(guid.to_string());
        if let Err(error) = tokio::fs::remove_dir_all(&dir).await
            && error.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!("mktranscode: hls cleanup failed for {guid} ({error})");
        }
    }

    /// Starts (or restarts) the HLS session for `guid`. Returns the path of
    /// the master playlist.
    pub async fn start(&mut self, guid: Uuid, media_path: &str) -> Result<String, String> {
        // Restarting a media item starts from a clean slate.
        self.remove_session(&guid).await;

        let dir = Path::new(HLS_ROOT).join(guid.to_string());
        if let Err(error) = tokio::fs::create_dir_all(&dir).await {
            return Err(format!("create session directory: {error}"));
        }

        let source = probe_source_dimensions(media_path).await;
        let renditions = plan_renditions(source.map(|(_, height)| height));

        let mut session = HlsSession {
            children: Vec::new(),
        };
        for rendition in &renditions {
            let args = ffmpeg_args(media_path, &dir, rendition);
            let child = Command::new("ffmpeg")
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|error| {
                    for spawned in session.children.iter_mut() {
                        let _ = spawned.start_kill();
                    }
                    // Synchronous context: the cleanup must complete here.
                    let _ = std::fs::remove_dir_all(&dir);
                    format!("spawn ffmpeg: {error}")
                })?;
            let rendition_name = if rendition.height > 0 {
                format!("{height}p", height = rendition.height)
            } else {
                "native".to_string()
            };
            eprintln!("mktranscode: hls rendition started for {guid} ({rendition_name})");
            session.children.push(child);
        }

        let master_body = master_playlist_body(&renditions, source);
        let master_path = dir.join("master.m3u8");
        if let Err(error) = tokio::fs::write(&master_path, master_body).await {
            for child in session.children.iter_mut() {
                let _ = child.start_kill();
            }
            let _ = tokio::fs::remove_dir_all(&dir).await;
            return Err(format!("write master playlist: {error}"));
        }

        self.sessions.insert(guid, session);
        self.sweep_expired().await;
        Ok(dir.join("master.m3u8").to_string_lossy().into_owned())
    }

    /// Stops the session for `guid` and deletes its segment directory.
    /// Returns true when a live session was actually running.
    pub async fn stop(&mut self, guid: &Uuid) -> bool {
        let Some(mut session) = self.sessions.remove(guid) else {
            // No live session (e.g. the pod restarted while the browser was
            // still playing); still remove the on-disk segments so a stale
            // page close frees the working set instead of waiting for the
            // age sweep.
            let dir = Path::new(HLS_ROOT).join(guid.to_string());
            if let Err(error) = tokio::fs::remove_dir_all(&dir).await
                && error.kind() != std::io::ErrorKind::NotFound
            {
                eprintln!("mktranscode: hls cleanup failed for {guid} ({error})");
            }
            return false;
        };
        Self::kill_session(&mut session);
        // The directory is removed even without a live session (e.g. the
        // pod restarted while the browser was still playing) so a stale
        // page close frees the working set instead of waiting for the age
        // sweep.
        let dir = Path::new(HLS_ROOT).join(guid.to_string());
        if let Err(error) = tokio::fs::remove_dir_all(&dir).await
            && error.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!("mktranscode: hls cleanup failed for {guid} ({error})");
        }
        true
    }

    /// Reaps exited ffmpeg children. Sessions whose children all exited
    /// normally (end of media) keep their segments until a stop message or
    /// the age sweep removes them.
    pub async fn reap(&mut self) {
        for (guid, session) in self.sessions.iter_mut() {
            session.children.retain_mut(|child| match child.try_wait() {
                Ok(Some(status)) => {
                    eprintln!("mktranscode: hls ffmpeg for {guid} exited with {status}");
                    false
                }
                Ok(None) => true,
                Err(error) => {
                    eprintln!("mktranscode: hls wait failed for {guid} ({error})");
                    true
                }
            });
        }
        self.sweep_expired().await;
    }

    /// Deletes session directories that were not written to within
    /// `HLS_SWEEP_AGE`, covering stops that never arrived (browser crash,
    /// pod restart). Active sessions are protected: segment writes keep the
    /// directory mtime fresh.
    async fn sweep_expired(&mut self) {
        if self.last_sweep.elapsed() < HLS_SWEEP_INTERVAL {
            return;
        }
        self.last_sweep = Instant::now();
        let Ok(mut entries) = tokio::fs::read_dir(HLS_ROOT).await else {
            return;
        };
        while let Some(entry) = match entries.next_entry().await {
            Ok(Some(entry)) => Some(entry),
            _ => None,
        } {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(age) = directory_age(&path).await
                && age > HLS_SWEEP_AGE
            {
                eprintln!(
                    "mktranscode: hls sweep removing stale directory {:?} (age {:?})",
                    path, age
                );
                let _ = tokio::fs::remove_dir_all(&path).await;
            }
        }
    }

    /// Terminates every running session. Used on shutdown; segment
    /// directories are left for the next consumer's age sweep.
    pub fn shutdown(&mut self) {
        for (guid, session) in self.sessions.iter_mut() {
            eprintln!("mktranscode: hls session {guid} terminated on shutdown");
            Self::kill_session(session);
        }
    }
}

impl Default for HlsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendition_plan_ladder() {
        assert_eq!(plan_renditions(Some(2160)).len(), 2);
        assert_eq!(plan_renditions(Some(1440))[0].height, 1080);
        assert_eq!(plan_renditions(Some(1080)).len(), 2);
        assert_eq!(plan_renditions(Some(900))[0].height, 900);
        assert_eq!(plan_renditions(Some(720)).len(), 1);
        assert_eq!(plan_renditions(Some(360))[0].height, 0);
        assert_eq!(plan_renditions(None).len(), 1);
    }

    #[test]
    fn media_path_validation() {
        assert!(is_supported_media_path(
            "smb://192.168.50.50/video/movies/a.mkv"
        ));
        assert!(is_supported_media_path(
            "nfs://192.168.50.50/media/movies/a.mkv"
        ));
        assert!(is_supported_media_path(
            "/mediakraken/metadata/movies/a.mkv"
        ));
        assert!(!is_supported_media_path(""));
        assert!(!is_supported_media_path("smb:/192.168.50.50/video/a.mkv"));
        assert!(!is_supported_media_path("http://192.168.50.50/video/a.mkv"));
        assert!(!is_supported_media_path("C:\\media\\a.mkv"));
    }

    #[test]
    fn rendition_plan_labels() {
        let ladder = plan_renditions(Some(2160));
        assert_eq!(ladder[0].label(), "1080");
        assert_eq!(ladder[1].label(), "720");
        assert_eq!(plan_renditions(Some(360))[0].label(), "src");
    }

    #[test]
    fn ffmpeg_args_build_scale_and_output_paths() {
        let dir = Path::new("/mediakraken/hls/guid");
        let args = ffmpeg_args(
            "/mediakraken/metadata/mov/a.mkv",
            dir,
            &plan_renditions(Some(2160))[0],
        );
        let joined = args.join(" ");
        assert!(joined.contains(r"scale=-2:min(1080\,ih)"));
        assert!(joined.contains("/mediakraken/hls/guid/1080_seg_%05d.ts"));
        assert!(joined.ends_with("/mediakraken/hls/guid/1080.m3u8"));
        assert!(joined.contains("-map 0:a?"));
        assert!(joined.contains("independent_segments"));

        let native = ffmpeg_args(
            "/mediakraken/metadata/mov/a.mkv",
            dir,
            &plan_renditions(Some(360))[0],
        );
        assert!(!native.iter().any(|arg| arg == "-vf"));
        assert!(native.join(" ").ends_with("/mediakraken/hls/guid/src.m3u8"));
    }

    #[test]
    fn master_playlist_lists_all_renditions() {
        let body = master_playlist_body(&plan_renditions(Some(2160)), Some((3840, 2160)));
        assert!(body.starts_with("#EXTM3U"));
        assert!(body.contains("#EXT-X-INDEPENDENT-SEGMENTS"));
        assert!(body.contains("RESOLUTION=1920x1080"));
        assert!(body.contains("RESOLUTION=1280x720"));
        assert!(body.contains("1080.m3u8"));
        assert!(body.contains("720.m3u8"));
    }

    #[test]
    fn master_playlist_single_rendition_without_dimensions() {
        let body = master_playlist_body(&plan_renditions(None), None);
        assert!(!body.contains("RESOLUTION"));
        assert!(body.contains("src.m3u8"));
    }
}
