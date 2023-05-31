// https://github.com/maxjoehnk/soundcloud-rs

use soundcloud::Client;
use soundcloud::Track;
use tokio::fs::File;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn provider_soundcloud_client(
    soundcloud_client_id: String,
) -> Result<Client, std::io::Error> {
    let client_id = std::env::var(soundcloud_client_id).unwrap();
    let client = Client::new(&client_id);
    Ok(client)
}

pub async fn provider_soundcloud_search(
    soundcloud_client: Client,
    band_name: String,
) -> Result<Vec<Track>, std::io::Error> {
    let tracks = soundcloud_client
        .tracks()
        .query(Some(band_name))
        .get()
        .await
        .unwrap();
    Ok(tracks)
}

pub async fn provider_soundcloud_track_download(
    soundcloud_client: Client,
    band_name: String,
    tracks: Vec<Track>,
) -> Result<(), std::io::Error> {
    for track in &tracks {
        if !track.downloadable {
            continue;
        }
        let track_title = track
            .title
            .to_string()
            .replace(&['\"', '.', '\'', '\\', '/', '?', '*'][..], "");
        let path = format!("{} - {}", band_name, track_title);
        let mut outfile = File::create(&path).await?.compat_write();
        if let Ok(num_bytes) = soundcloud_client.download(track, &mut outfile).await {
            if num_bytes > 0 {
                break;
            }
        }
    }
    Ok(())
}
