use crossbeam_channel::unbounded;
use fltk::{
    app,
    button::Button,
    dialog::{FileDialogType, NativeFileChooser, alert_default, message_default},
    prelude::*,
    window::Window,
};
use fltk_theme::{SchemeType, WidgetScheme};
use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

const THEATER_THIN_API_ADDR: &str = "127.0.0.1:7878";

#[derive(Debug, Clone, Copy)]
enum UiMsg {
    ShowSettings,
    ShowMenu,
    PlaySelected,
    Pause,
    Resume,
    Stop,
}

fn make_image_button(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    png_bytes: &[u8],
) -> Result<Button, Box<dyn Error>> {
    let mut button = Button::new(x, y, w, h, None);
    let mut image = fltk::image::PngImage::from_data(png_bytes)?;
    image.scale(w, h, true, true);
    button.set_image(Some(image));
    Ok(button)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load images
    let bytes_image_rectangle =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/rectangles_black.png");
    let bytes_image_new = include_bytes!("../../../docker/core/mkwebaxum/static/image/new.png");
    let bytes_image_movie_ticket =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/movie_ticket.png");
    let bytes_image_television =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/television.png");
    let bytes_image_vid_game =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/vid_game.png");
    let bytes_image_theater =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/theater.png");
    let bytes_image_headphone =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/headphone.png");
    let bytes_image_television_live =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/television_live.png");
    let bytes_image_vid_camera =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/vid_camera.png");
    let bytes_image_earth = include_bytes!("../../../docker/core/mkwebaxum/static/image/earth.png");
    let bytes_image_music_video = include_bytes!(
        "../../../docker/core/mkwebaxum/static/image/listening-music-video-clip-with-auricular.png"
    );
    let bytes_image_photo = include_bytes!("../../../docker/core/mkwebaxum/static/image/photo.png");
    let bytes_image_radio = include_bytes!("../../../docker/core/mkwebaxum/static/image/radio.png");
    let bytes_image_books = include_bytes!("../../../docker/core/mkwebaxum/static/image/books.png");
    let bytes_image_settings =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/settings.png");
    let bytes_image_return =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/navigation/return.png");

    let _server_list =
        mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();

    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();

    let (sender, receiver) = unbounded::<UiMsg>();

    // Menu window
    let mut window_menu = Window::default().with_size(800, 480);

    let _button_in_progress = make_image_button(0, 0, 133, 96, bytes_image_rectangle)?;
    let _button_new = make_image_button(0, 96, 133, 96, bytes_image_new)?;
    let _button_movie = make_image_button(0, 192, 133, 96, bytes_image_movie_ticket)?;
    let _button_tv = make_image_button(0, 288, 133, 96, bytes_image_television)?;
    let _button_game = make_image_button(0, 384, 133, 96, bytes_image_vid_game)?;

    let mut button_demo = make_image_button(133, 0, 532, 384, bytes_image_theater)?;

    let _button_music = make_image_button(133, 384, 133, 96, bytes_image_headphone)?;
    let _button_live_tv = make_image_button(266, 384, 133, 96, bytes_image_television_live)?;
    let _button_home_video = make_image_button(399, 384, 133, 96, bytes_image_vid_camera)?;
    let _button_internet = make_image_button(532, 384, 133, 96, bytes_image_earth)?;

    let _button_music_video = make_image_button(666, 0, 133, 96, bytes_image_music_video)?;
    let _button_pictures = make_image_button(666, 96, 133, 96, bytes_image_photo)?;
    let _button_radio = make_image_button(666, 192, 133, 96, bytes_image_radio)?;
    let _button_books = make_image_button(666, 288, 133, 96, bytes_image_books)?;
    let mut button_settings = make_image_button(666, 384, 133, 96, bytes_image_settings)?;

    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);
    window_menu.show();

    // Settings window
    let mut window_settings = Window::default().with_size(800, 480);
    let mut button_pause = Button::new(133, 96, 532, 64, "Pause");
    let mut button_resume = Button::new(133, 192, 532, 64, "Resume");
    let mut button_stop = Button::new(133, 288, 532, 64, "Stop");
    let mut button_settings_back = make_image_button(666, 384, 133, 96, bytes_image_return)?;

    button_pause.set_tooltip("Pause playback on theater_thin");
    button_resume.set_tooltip("Resume playback on theater_thin");
    button_stop.set_tooltip("Stop playback on theater_thin");

    window_settings.end();
    window_settings.make_resizable(true);
    window_settings.fullscreen(true);
    window_settings.hide();

    {
        let sender = sender.clone();
        button_settings.set_callback(move |_| {
            let _ = sender.send(UiMsg::ShowSettings);
        });
    }

    {
        let sender = sender.clone();
        button_demo.set_callback(move |_| {
            let _ = sender.send(UiMsg::PlaySelected);
        });
    }

    {
        let sender = sender.clone();
        button_pause.set_callback(move |_| {
            let _ = sender.send(UiMsg::Pause);
        });
    }

    {
        let sender = sender.clone();
        button_resume.set_callback(move |_| {
            let _ = sender.send(UiMsg::Resume);
        });
    }

    {
        let sender = sender.clone();
        button_stop.set_callback(move |_| {
            let _ = sender.send(UiMsg::Stop);
        });
    }

    {
        let sender = sender.clone();
        button_settings_back.set_callback(move |_| {
            let _ = sender.send(UiMsg::ShowMenu);
        });
    }

    while app.wait() {
        if let Ok(msg) = receiver.try_recv() {
            match msg {
                UiMsg::ShowSettings => {
                    window_menu.hide();
                    window_settings.show();
                }
                UiMsg::ShowMenu => {
                    window_settings.hide();
                    window_menu.show();
                }
                UiMsg::PlaySelected => {
                    if let Some(path) = pick_media_file() {
                        if Path::new(&path).exists() {
                            let encoded_path = url_encode(&path);
                            let endpoint = format!("/api/play?path={encoded_path}");
                            match send_theater_thin_post(&endpoint) {
                                Ok((status, body)) if status.starts_with("HTTP/1.1 200") => {
                                    message_default(&format!("theater_thin playing:\n{path}"));
                                }
                                Ok((status, body)) => {
                                    alert_default(&format!(
                                        "theater_thin play failed:\n{status}\n{body}"
                                    ));
                                }
                                Err(error) => {
                                    alert_default(&format!(
                                        "Unable to reach theater_thin API:\n{error}"
                                    ));
                                }
                            }
                        } else {
                            alert_default("Selected file does not exist.");
                        }
                    }
                }
                UiMsg::Pause => handle_simple_api("/api/pause", "Paused", "pause"),
                UiMsg::Resume => handle_simple_api("/api/resume", "Resumed", "resume"),
                UiMsg::Stop => handle_simple_api("/api/stop", "Stopped", "stop"),
            }
        }
    }

    Ok(())
}

fn pick_media_file() -> Option<String> {
    let mut chooser = NativeFileChooser::new(FileDialogType::BrowseFile);
    chooser.set_title("Select media file to play in theater_thin");
    chooser.set_filter("*.*");
    chooser.show();

    let selected = chooser.filename();
    if selected.as_os_str().is_empty() {
        None
    } else {
        Some(selected.to_string_lossy().into_owned())
    }
}

fn handle_simple_api(endpoint: &str, success_title: &str, action_name: &str) {
    match send_theater_thin_post(endpoint) {
        Ok((status, body)) if status.starts_with("HTTP/1.1 200") => {
            message_default(&format!("{success_title} theater_thin playback."));
        }
        Ok((status, body)) => {
            alert_default(&format!(
                "Failed to {action_name} on theater_thin:\n{status}\n{body}"
            ));
        }
        Err(error) => {
            alert_default(&format!("Unable to reach theater_thin API:\n{error}"));
        }
    }
}

fn send_theater_thin_post(endpoint: &str) -> Result<(String, String), String> {
    let mut stream = TcpStream::connect(THEATER_THIN_API_ADDR)
        .map_err(|error| format!("connect {THEATER_THIN_API_ADDR} failed: {error}"))?;

    let request = format!(
        "POST {endpoint} HTTP/1.1\r\nHost: {THEATER_THIN_API_ADDR}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
    );

    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write request failed: {error}"))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| format!("read response failed: {error}"))?;

    let mut sections = response.splitn(2, "\r\n\r\n");
    let header = sections.next().unwrap_or_default();
    let body = sections.next().unwrap_or_default().to_string();
    let status_line = header
        .lines()
        .next()
        .unwrap_or("HTTP/1.1 500 Invalid response");

    Ok((status_line.to_string(), body))
}

fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        let is_unreserved = matches!(
            byte,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~'
        );

        if is_unreserved {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
