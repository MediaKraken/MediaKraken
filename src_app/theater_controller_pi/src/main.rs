use crossbeam_channel::{Sender, unbounded};
use fltk::{
    app,
    button::Button,
    dialog::{FileDialogType, NativeFileChooser, alert_default, message_default},
    prelude::*,
    window::Window,
};
use fltk_theme::{SchemeType, WidgetScheme};
use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::Path,
    time::Duration,
};

const DEFAULT_THEATER_THIN_API_ADDR: &str = "127.0.0.1:7878";
const THEATER_THIN_API_ADDR_ENV: &str = "THEATER_THIN_API_ADDR";
const TCP_TIMEOUT: Duration = Duration::from_secs(5);

const FILTER_VIDEO: &str = "Videos\t*.{mp4,mkv,avi,mov,webm,m4v,ts,wmv,flv,mpg,mpeg}";
const FILTER_AUDIO: &str = "Audio\t*.{mp3,flac,ogg,wav,m4a,aac,opus,wma}";
const FILTER_IMAGE: &str = "Images\t*.{png,jpg,jpeg,gif,bmp,webp,tiff}";
const FILTER_ANY: &str = "All files\t*";

#[derive(Debug, Clone, Copy)]
enum MediaCategory {
    InProgress,
    New,
    Movie,
    Tv,
    Game,
    Theater,
    Music,
    LiveTv,
    HomeVideo,
    Internet,
    MusicVideo,
    Pictures,
    Radio,
    Books,
}

impl MediaCategory {
    fn label(self) -> &'static str {
        match self {
            Self::InProgress => "In Progress",
            Self::New => "New Additions",
            Self::Movie => "Movies",
            Self::Tv => "TV Shows",
            Self::Game => "Games",
            Self::Theater => "Any Media",
            Self::Music => "Music",
            Self::LiveTv => "Live TV",
            Self::HomeVideo => "Home Video",
            Self::Internet => "Internet",
            Self::MusicVideo => "Music Videos",
            Self::Pictures => "Pictures",
            Self::Radio => "Radio",
            Self::Books => "Books",
        }
    }

    fn filter(self) -> Option<&'static str> {
        match self {
            Self::InProgress
            | Self::New
            | Self::Movie
            | Self::Tv
            | Self::HomeVideo
            | Self::MusicVideo => Some(FILTER_VIDEO),
            Self::Theater => Some(FILTER_ANY),
            Self::Music => Some(FILTER_AUDIO),
            Self::Pictures => Some(FILTER_IMAGE),
            Self::Game | Self::LiveTv | Self::Internet | Self::Radio | Self::Books => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum UiMsg {
    ShowSettings,
    ShowMenu,
    PlayCategory(MediaCategory),
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

fn wire_category(button: &mut Button, sender: &Sender<UiMsg>, category: MediaCategory) {
    button.set_tooltip(category.label());
    let sender = sender.clone();
    button.set_callback(move |_| {
        let _ = sender.send(UiMsg::PlayCategory(category));
    });
}

fn wire_msg(button: &mut Button, sender: &Sender<UiMsg>, msg: UiMsg) {
    let sender = sender.clone();
    button.set_callback(move |_| {
        let _ = sender.send(msg);
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_addr = env::var(THEATER_THIN_API_ADDR_ENV)
        .unwrap_or_else(|_| DEFAULT_THEATER_THIN_API_ADDR.to_string());

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

    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();

    let (sender, receiver) = unbounded::<UiMsg>();

    let mut window_menu = Window::default().with_size(800, 480);

    let mut button_in_progress = make_image_button(0, 0, 133, 96, bytes_image_rectangle)?;
    let mut button_new = make_image_button(0, 96, 133, 96, bytes_image_new)?;
    let mut button_movie = make_image_button(0, 192, 133, 96, bytes_image_movie_ticket)?;
    let mut button_tv = make_image_button(0, 288, 133, 96, bytes_image_television)?;
    let mut button_game = make_image_button(0, 384, 133, 96, bytes_image_vid_game)?;

    let mut button_theater = make_image_button(133, 0, 532, 384, bytes_image_theater)?;

    let mut button_music = make_image_button(133, 384, 133, 96, bytes_image_headphone)?;
    let mut button_live_tv = make_image_button(266, 384, 133, 96, bytes_image_television_live)?;
    let mut button_home_video = make_image_button(399, 384, 133, 96, bytes_image_vid_camera)?;
    let mut button_internet = make_image_button(532, 384, 133, 96, bytes_image_earth)?;

    let mut button_music_video = make_image_button(666, 0, 133, 96, bytes_image_music_video)?;
    let mut button_pictures = make_image_button(666, 96, 133, 96, bytes_image_photo)?;
    let mut button_radio = make_image_button(666, 192, 133, 96, bytes_image_radio)?;
    let mut button_books = make_image_button(666, 288, 133, 96, bytes_image_books)?;
    let mut button_settings = make_image_button(666, 384, 133, 96, bytes_image_settings)?;

    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);
    window_menu.show();

    let mut window_playback = Window::default().with_size(800, 480);
    let mut button_pause = Button::new(133, 96, 532, 64, "Pause");
    let mut button_resume = Button::new(133, 192, 532, 64, "Resume");
    let mut button_stop = Button::new(133, 288, 532, 64, "Stop");
    let mut button_playback_back = make_image_button(666, 384, 133, 96, bytes_image_return)?;

    button_pause.set_tooltip("Pause playback on theater_thin");
    button_resume.set_tooltip("Resume playback on theater_thin");
    button_stop.set_tooltip("Stop playback on theater_thin");
    button_playback_back.set_tooltip("Back to menu");

    window_playback.end();
    window_playback.make_resizable(true);
    window_playback.fullscreen(true);
    window_playback.hide();

    wire_category(&mut button_in_progress, &sender, MediaCategory::InProgress);
    wire_category(&mut button_new, &sender, MediaCategory::New);
    wire_category(&mut button_movie, &sender, MediaCategory::Movie);
    wire_category(&mut button_tv, &sender, MediaCategory::Tv);
    wire_category(&mut button_game, &sender, MediaCategory::Game);
    wire_category(&mut button_theater, &sender, MediaCategory::Theater);
    wire_category(&mut button_music, &sender, MediaCategory::Music);
    wire_category(&mut button_live_tv, &sender, MediaCategory::LiveTv);
    wire_category(&mut button_home_video, &sender, MediaCategory::HomeVideo);
    wire_category(&mut button_internet, &sender, MediaCategory::Internet);
    wire_category(&mut button_music_video, &sender, MediaCategory::MusicVideo);
    wire_category(&mut button_pictures, &sender, MediaCategory::Pictures);
    wire_category(&mut button_radio, &sender, MediaCategory::Radio);
    wire_category(&mut button_books, &sender, MediaCategory::Books);

    button_settings.set_tooltip("Playback controls");
    wire_msg(&mut button_settings, &sender, UiMsg::ShowSettings);

    wire_msg(&mut button_pause, &sender, UiMsg::Pause);
    wire_msg(&mut button_resume, &sender, UiMsg::Resume);
    wire_msg(&mut button_stop, &sender, UiMsg::Stop);
    wire_msg(&mut button_playback_back, &sender, UiMsg::ShowMenu);

    while app.wait() {
        if let Ok(msg) = receiver.try_recv() {
            match msg {
                UiMsg::ShowSettings => {
                    window_menu.hide();
                    window_playback.show();
                }
                UiMsg::ShowMenu => {
                    window_playback.hide();
                    window_menu.show();
                }
                UiMsg::PlayCategory(category) => handle_play_category(category, &api_addr),
                UiMsg::Pause => handle_simple_api(&api_addr, "/api/pause", "Paused", "pause"),
                UiMsg::Resume => handle_simple_api(&api_addr, "/api/resume", "Resumed", "resume"),
                UiMsg::Stop => handle_simple_api(&api_addr, "/api/stop", "Stopped", "stop"),
            }
        }
    }

    Ok(())
}

fn handle_play_category(category: MediaCategory, api_addr: &str) {
    let Some(filter) = category.filter() else {
        alert_default(&format!(
            "{} playback is not yet supported from theater_controller_pi.",
            category.label()
        ));
        return;
    };

    let Some(path) = pick_media_file(category.label(), filter) else {
        return;
    };

    if !Path::new(&path).exists() {
        alert_default("Selected file does not exist.");
        return;
    }

    let endpoint = format!("/api/play?path={}", url_encode(&path));
    match send_theater_thin_post(api_addr, &endpoint) {
        Ok((200, _)) => {
            message_default(&format!("theater_thin playing:\n{path}"));
        }
        Ok((code, body)) => {
            alert_default(&format!(
                "theater_thin play failed:\nHTTP {code}\n{body}"
            ));
        }
        Err(error) => {
            alert_default(&format!("Unable to reach theater_thin API:\n{error}"));
        }
    }
}

fn pick_media_file(category_label: &str, filter: &str) -> Option<String> {
    let mut chooser = NativeFileChooser::new(FileDialogType::BrowseFile);
    chooser.set_title(&format!("Select {category_label} file to play in theater_thin"));
    chooser.set_filter(filter);
    chooser.show();

    let selected = chooser.filename();
    if selected.as_os_str().is_empty() {
        None
    } else {
        Some(selected.to_string_lossy().into_owned())
    }
}

fn handle_simple_api(api_addr: &str, endpoint: &str, success_title: &str, action_name: &str) {
    match send_theater_thin_post(api_addr, endpoint) {
        Ok((200, _)) => {
            message_default(&format!("{success_title} theater_thin playback."));
        }
        Ok((code, body)) => {
            alert_default(&format!(
                "Failed to {action_name} on theater_thin:\nHTTP {code}\n{body}"
            ));
        }
        Err(error) => {
            alert_default(&format!("Unable to reach theater_thin API:\n{error}"));
        }
    }
}

fn send_theater_thin_post(api_addr: &str, endpoint: &str) -> Result<(u16, String), String> {
    let socket_addr = api_addr
        .to_socket_addrs()
        .map_err(|error| format!("resolve {api_addr} failed: {error}"))?
        .next()
        .ok_or_else(|| format!("no addresses resolved for {api_addr}"))?;

    let mut stream = TcpStream::connect_timeout(&socket_addr, TCP_TIMEOUT)
        .map_err(|error| format!("connect {api_addr} failed: {error}"))?;
    stream
        .set_read_timeout(Some(TCP_TIMEOUT))
        .map_err(|error| format!("set read timeout failed: {error}"))?;
    stream
        .set_write_timeout(Some(TCP_TIMEOUT))
        .map_err(|error| format!("set write timeout failed: {error}"))?;

    let request = format!(
        "POST {endpoint} HTTP/1.1\r\nHost: {api_addr}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
    );

    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write request failed: {error}"))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| format!("read response failed: {error}"))?;

    parse_http_response(&response)
}

fn parse_http_response(response: &str) -> Result<(u16, String), String> {
    let mut sections = response.splitn(2, "\r\n\r\n");
    let header = sections.next().unwrap_or_default();
    let body = sections.next().unwrap_or_default().to_string();
    let status_line = header
        .lines()
        .next()
        .ok_or_else(|| "empty response".to_string())?;
    let code_str = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| format!("malformed status line: {status_line}"))?;
    let code: u16 = code_str
        .parse()
        .map_err(|_| format!("invalid status code: {code_str}"))?;
    Ok((code, body))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encode_leaves_unreserved_chars_alone() {
        assert_eq!(url_encode("abcXYZ-._~"), "abcXYZ-._~");
    }

    #[test]
    fn url_encode_percent_encodes_reserved_chars() {
        assert_eq!(url_encode("/path to/file"), "%2Fpath%20to%2Ffile");
        assert_eq!(url_encode("a+b&c=d"), "a%2Bb%26c%3Dd");
    }

    #[test]
    fn url_encode_handles_non_ascii() {
        // U+00E9 é -> UTF-8 bytes 0xC3 0xA9
        assert_eq!(url_encode("caf\u{00e9}"), "caf%C3%A9");
    }

    #[test]
    fn parse_http_response_extracts_code_and_body() {
        let raw = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\":true}";
        let (code, body) = parse_http_response(raw).expect("parse");
        assert_eq!(code, 200);
        assert_eq!(body, "{\"ok\":true}");
    }

    #[test]
    fn parse_http_response_handles_http_1_0() {
        let raw = "HTTP/1.0 404 Not Found\r\n\r\nnope";
        let (code, body) = parse_http_response(raw).expect("parse");
        assert_eq!(code, 404);
        assert_eq!(body, "nope");
    }

    #[test]
    fn parse_http_response_rejects_malformed_status() {
        assert!(parse_http_response("").is_err());
        assert!(parse_http_response("HTTP/1.1\r\n\r\n").is_err());
        assert!(parse_http_response("HTTP/1.1 NOPE OK\r\n\r\n").is_err());
    }

    #[test]
    fn media_category_filters_are_consistent() {
        assert!(MediaCategory::Movie.filter().is_some());
        assert!(MediaCategory::Music.filter().is_some());
        assert!(MediaCategory::Pictures.filter().is_some());
        assert!(MediaCategory::Theater.filter().is_some());
        assert!(MediaCategory::Game.filter().is_none());
        assert!(MediaCategory::Radio.filter().is_none());
        assert!(MediaCategory::Books.filter().is_none());
        assert!(MediaCategory::Internet.filter().is_none());
        assert!(MediaCategory::LiveTv.filter().is_none());
    }
}
