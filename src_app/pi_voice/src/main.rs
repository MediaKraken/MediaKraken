use fltk::{
    app, app::*, button::*, enums::*, frame::*, group::*, input::*, output::Output, prelude::*,
    window::*,
};
use fltk_webview::*;
mod choice;
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::error::Error;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Stop,
    Recognise,
}

pub mod record {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{FromSample, Sample};
    use hound::WavWriter;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    pub struct Recorder {
        utils: Option<(Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>, cpal::Stream)>,
    }

    impl Recorder {
        pub fn new() -> Self {
            Recorder { utils: None }
        }
        pub fn start_recording(&mut self) -> Result<(), anyhow::Error> {
            if self.utils.is_some() {
                return Err(anyhow::Error::msg(
                    "Attempted to start recording when already recording!",
                ));
            }

            let host = cpal::default_host();

            // Set up the input device and stream with the default input config.
            let device = host.default_input_device().unwrap();

            println!("Input device: {}", device.name()?);

            let config = device
                .default_input_config()
                .expect("Failed to get default input config");
            println!("Default input config: {:?}", config);

            // The WAV file we're recording to.
            let spec = wav_spec_from_config(&config);
            let writer = hound::WavWriter::create(Path::new("voice_file.wav"), spec)?;
            let writer = Arc::new(Mutex::new(Some(writer)));

            // Run the input stream on a separate thread.
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
                    )))
                }
            };
            // ========================
            stream.play()?;
            self.utils = Some((writer, stream));
            Ok(())
        }

        pub fn stop_recording(&mut self) -> Result<(), anyhow::Error> {
            match self.utils.take() {
                Some((writer, stream)) => {
                    stream.pause()?;
                    if let Some(writer) = writer.lock().ok().and_then(|mut lock| lock.take()) {
                        writer.finalize()?;
                    }
                    Ok(())
                }
                None => {
                    return Err(anyhow::Error::msg(
                        "Attempted to stop recording when not recording!",
                    ));
                }
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
        if let Ok(mut guard) = writer.try_lock() {
            if let Some(writer) = guard.as_mut() {
                for &sample in input.iter() {
                    let sample: U = U::from_sample(sample);
                    writer.write_sample(sample).ok();
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    use record::Recorder;
    let mut recorder = Recorder::new();

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

    // setup the event
    let (s, r) = app::channel::<Message>();

    window_main.end();
    window_main.show();
    window_main.make_current();

    let mut wv = Webview::create(false, &mut wv_win);
    wv.navigate("https://mkprod:8900/api");

    button_start_record_loop.set_callback(move |_| {
        s.send(Message::Start);
    });

    button_stop_record_loop.set_callback(move |_| {
        s.send(Message::Stop);
    });

    button_stop_and_recognise.set_callback(move |_| {
        s.send(Message::Recognise);
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Start => {
                    println!("Start");
                    if let Err(err) = recorder.start_recording() {
                        eprintln!("failed to start recording: {err}");
                    }
                }
                Message::Stop => {
                    println!("Stop");
                    if let Err(err) = recorder.stop_recording() {
                        eprintln!("failed to stop recording: {err}");
                    }
                }
                Message::Recognise => {
                    println!("Recognise");
                    if let Err(err) = recorder.stop_recording() {
                        eprintln!("failed to stop recording before recognition: {err}");
                    }
                    // convert wav to proper format via ffmpeg
                    let output = Command::new("ffmpeg")
                        .args([
                            "-y",
                            "-i",
                            "voice_file.wav",
                            "-ar",
                            "16000",
                            "-ac",
                            "1",
                            "voice_file_mono.wav",
                        ])
                        .stdout(Stdio::piped())
                        .output()?;
                    let stdout: String = String::from_utf8(output.stdout)?;
                    println!("{}", stdout);
                    // speech rec via vosk
                    let output = Command::new("python3")
                        .args(["send_wav_to_websocket.py"])
                        .stdout(Stdio::piped())
                        .output()?;
                    let stdout = String::from_utf8(output.stdout)?;
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
                    // push output to page
                    let search_query = search_str.trim().replace(' ', "%20");
                    if !search_query.is_empty() {
                        wv.navigate(
                            format!("https://mkprod:8900/api/titlesearch/{search_query}").as_str(),
                        );
                    } else {
                        eprintln!("no recognized text found in websocket output");
                    }
                }
            }
        }
    }
    Ok(())
}
