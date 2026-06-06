use clap::Parser;
use fltk::{
    app, button::Button, enums::FrameType, output::Output, prelude::*, window::Window,
};
use fltk_webview::Webview;
use fltk_webview::FromFltkWindow;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;

mod choice;

#[derive(Parser, Debug, Clone)]
#[command(name = "pi_voice", version, about = "MediaKraken voice capture client")]
struct Cli {
    /// Base URL of the MediaKraken API (used by the embedded webview and title search).
    #[arg(long, default_value = "https://mkprod:8900")]
    api_base: String,

    /// Vosk WebSocket URI used by the bundled Python client.
    #[arg(long, default_value = "ws://mkprod:2700")]
    vosk_uri: String,

    /// Working directory for intermediate WAV files and the Python helper.
    #[arg(long)]
    workdir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Recognise,
    Status(String),
    Navigate(String),
}

pub mod record {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{FromSample, Sample};
    use hound::WavWriter;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    pub struct Recorder {
        output_path: PathBuf,
        utils: Option<(Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>, cpal::Stream)>,
    }

    impl Recorder {
        pub fn new(output_path: PathBuf) -> Self {
            Recorder {
                output_path,
                utils: None,
            }
        }

        pub fn start_recording(&mut self) -> Result<(), anyhow::Error> {
            if self.utils.is_some() {
                return Err(anyhow::Error::msg(
                    "Attempted to start recording when already recording!",
                ));
            }

            let host = cpal::default_host();
            let device = host
                .default_input_device()
                .ok_or_else(|| anyhow::Error::msg("no default input device available"))?;

            println!("Input device: {}", device.name()?);

            let config = device.default_input_config()?;
            println!("Default input config: {:?}", config);

            let spec = wav_spec_from_config(&config);
            let writer = hound::WavWriter::create(&self.output_path, spec)?;
            let writer = Arc::new(Mutex::new(Some(writer)));
            let writer_2 = writer.clone();

            let err_fn = move |err| {
                eprintln!("an error occurred on stream: {}", err);
            };

            let stream = match config.sample_format() {
                cpal::SampleFormat::I8 => device.build_input_stream(
                    &config.into(),
                    move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &config.into(),
                    move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::I32 => device.build_input_stream(
                    &config.into(),
                    move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.into(),
                    move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                    err_fn,
                    None,
                )?,
                sample_format => {
                    return Err(anyhow::Error::msg(format!(
                        "Unsupported sample format '{sample_format}'"
                    )));
                }
            };
            stream.play()?;
            self.utils = Some((writer, stream));
            Ok(())
        }

        /// Stop recording. Returns Ok(true) if a recording was stopped,
        /// Ok(false) if nothing was in progress.
        pub fn stop_recording(&mut self) -> Result<bool, anyhow::Error> {
            match self.utils.take() {
                Some((writer, stream)) => {
                    stream.pause()?;
                    if let Some(writer) = writer.lock().ok().and_then(|mut lock| lock.take()) {
                        writer.finalize()?;
                    }
                    Ok(true)
                }
                None => Ok(false),
            }
        }
    }

    fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
        if format.is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        }
    }

    fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
        hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: (config.sample_format().sample_size() * 8) as _,
            sample_format: sample_format(config.sample_format()),
        }
    }

    type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

    fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    where
        T: Sample,
        U: Sample + hound::Sample + FromSample<T>,
    {
        let Ok(mut guard) = writer.lock() else {
            eprintln!("recording writer lock poisoned; dropping samples");
            return;
        };
        let Some(writer) = guard.as_mut() else {
            return;
        };
        for &sample in input.iter() {
            let sample: U = U::from_sample(sample);
            if let Err(err) = writer.write_sample(sample) {
                eprintln!("failed to write audio sample: {}", err);
                return;
            }
        }
    }
}

/// Parse the stdout of the Python Vosk client into a single space-joined
/// search string.
fn extract_search_text(stdout: &str) -> String {
    let mut search_str = String::new();
    for line_item in stdout.lines() {
        let line_item = line_item.trim();
        let text = serde_json::from_str::<serde_json::Value>(line_item)
            .ok()
            .and_then(|value| {
                value
                    .get("text")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            })
            .or_else(|| {
                line_item
                    .strip_prefix("\"text\" :")
                    .map(|value| value.trim().trim_matches('"').to_owned())
            });

        if let Some(text) = text.filter(|text| !text.is_empty()) {
            if !search_str.is_empty() {
                search_str.push(' ');
            }
            search_str.push_str(text.as_str());
        }
    }
    search_str.trim().to_owned()
}

struct ProcessingPaths {
    raw_wav: PathBuf,
    mono_wav: PathBuf,
    script: PathBuf,
}

fn run_recognition(paths: &ProcessingPaths, vosk_uri: &str) -> Result<String, String> {
    let ffmpeg = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            paths.raw_wav.to_str().ok_or("raw wav path is not utf-8")?,
            "-ar",
            "16000",
            "-ac",
            "1",
            paths.mono_wav.to_str().ok_or("mono wav path is not utf-8")?,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|err| format!("failed to spawn ffmpeg: {err}"))?;

    if !ffmpeg.status.success() {
        return Err(format!(
            "ffmpeg failed ({}): {}",
            ffmpeg.status,
            String::from_utf8_lossy(&ffmpeg.stderr).trim()
        ));
    }

    let python = Command::new("python3")
        .arg(&paths.script)
        .arg(&paths.mono_wav)
        .arg(vosk_uri)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|err| format!("failed to spawn python3: {err}"))?;

    if !python.status.success() {
        return Err(format!(
            "vosk client failed ({}): {}",
            python.status,
            String::from_utf8_lossy(&python.stderr).trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&python.stdout);
    let text = extract_search_text(&stdout);
    if text.is_empty() {
        Err("no speech recognised".to_owned())
    } else {
        Ok(text)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let workdir = cli
        .workdir
        .clone()
        .or_else(|| std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)))
        .unwrap_or_else(|| PathBuf::from("."));

    let paths = Arc::new(ProcessingPaths {
        raw_wav: workdir.join("voice_file.wav"),
        mono_wav: workdir.join("voice_file_mono.wav"),
        script: workdir.join("send_wav_to_websocket.py"),
    });

    let api_base = cli.api_base.trim_end_matches('/').to_owned();
    let vosk_uri = cli.vosk_uri.clone();

    let mut recorder = record::Recorder::new(paths.raw_wav.clone());

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window_main = Window::default().with_size(1800, 960);

    let mut choice_media_type = choice::MyChoice::new(20, 20, 90, 30, None);
    choice_media_type.add_choices(&[
        "UHD",
        "BluRay",
        "DVD",
        "CD",
        "Book",
        "HDDVD",
        "LASERDISC",
        "GAME",
    ]);
    choice_media_type.set_current_choice(0);
    choice_media_type.button().set_frame(FrameType::BorderBox);
    choice_media_type.frame().set_frame(FrameType::BorderBox);

    let mut out = Output::new(20, 120, 1700, 120, "");
    out.set_text_size(20);
    out.set_value("Started");

    let mut button_start_record_loop = Button::new(210, 0, 133, 25, "Start Record");
    let mut button_stop_record_loop = Button::new(210, 40, 133, 25, "Stop Record");
    let mut button_stop_and_recognise = Button::new(210, 80, 133, 25, "Recognise");

    let mut wv_win = Window::new(20, 250, 1700, 700, "");

    let (s, r) = app::channel::<Message>();

    window_main.end();
    window_main.show();
    window_main.make_current();

    let wv = Webview::create(false, &mut wv_win);
    wv.navigate(&format!("{api_base}/api"));

    button_start_record_loop.set_callback({
        let s = s;
        move |_| s.send(Message::Start)
    });

    button_stop_record_loop.set_callback({
        let s = s;
        move |_| s.send(Message::Stop)
    });

    button_stop_and_recognise.set_callback({
        let s = s;
        move |_| s.send(Message::Recognise)
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Start => {
                    if let Err(err) = recorder.start_recording() {
                        let text = format!("failed to start recording: {err}");
                        eprintln!("{text}");
                        out.set_value(&text);
                    } else {
                        out.set_value("Recording...");
                    }
                }
                Message::Stop => {
                    match recorder.stop_recording() {
                        Ok(true) => out.set_value("Stopped"),
                        Ok(false) => out.set_value("Not recording"),
                        Err(err) => {
                            let text = format!("failed to stop recording: {err}");
                            eprintln!("{text}");
                            out.set_value(&text);
                        }
                    }
                }
                Message::Recognise => {
                    if let Err(err) = recorder.stop_recording() {
                        eprintln!("failed to stop recording before recognition: {err}");
                    }
                    if !paths.raw_wav.exists() {
                        out.set_value("No recording available");
                        continue;
                    }
                    out.set_value("Recognising...");

                    let media_type = choice_media_type.choice();
                    let paths = paths.clone();
                    let vosk_uri = vosk_uri.clone();
                    let api_base = api_base.clone();
                    let s = s;
                    thread::spawn(move || {
                        match run_recognition(&paths, &vosk_uri) {
                            Ok(text) => {
                                let query = urlencoding::encode(&text);
                                let url = if media_type.is_empty() {
                                    format!("{api_base}/api/titlesearch/{query}")
                                } else {
                                    let media = urlencoding::encode(&media_type);
                                    format!(
                                        "{api_base}/api/titlesearch/{query}?media_type={media}"
                                    )
                                };
                                s.send(Message::Status(format!("Recognised: {text}")));
                                s.send(Message::Navigate(url));
                            }
                            Err(err) => {
                                s.send(Message::Status(format!("Recognition failed: {err}")));
                            }
                        }
                    });
                }
                Message::Status(text) => {
                    println!("{text}");
                    out.set_value(&text);
                }
                Message::Navigate(url) => {
                    wv.navigate(&url);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::extract_search_text;

    #[test]
    fn extracts_text_from_json_lines() {
        let stdout = "{\"text\": \"hello world\"}\n{\"text\": \"second phrase\"}\n";
        assert_eq!(extract_search_text(stdout), "hello world second phrase");
    }

    #[test]
    fn skips_empty_text_entries() {
        let stdout = "{\"text\": \"\"}\n{\"text\": \"only one\"}\n";
        assert_eq!(extract_search_text(stdout), "only one");
    }

    #[test]
    fn falls_back_to_legacy_prefix() {
        let stdout = "\"text\" : \"legacy format\"\n";
        assert_eq!(extract_search_text(stdout), "legacy format");
    }

    #[test]
    fn returns_empty_on_no_matches() {
        assert_eq!(extract_search_text("{\"partial\": \"x\"}\n"), "");
    }
}
