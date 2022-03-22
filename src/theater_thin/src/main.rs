use fltk::{enums::Color, prelude::*, *};
use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use std::error::Error;
use argh::FromArgs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{copy, split, stdin as tokio_stdin, stdout as tokio_stdout, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
use crossbeam_channel::unbounded;

/// Tokio Rustls client
#[derive(FromArgs)]
struct Options {
    /// host
    #[argh(positional)]
    host: String,

    /// port
    #[argh(option, short = 'p', default = "443")]
    port: u16,

    /// domain
    #[argh(option, short = 'd')]
    domain: Option<String>,

    /// cafile
    #[argh(option, short = 'c')]
    cafile: Option<PathBuf>,
}

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");

    // Create inner window to act as embedded media player
    let mut mpv_win = window::Window::new(10, 10, 780, 520, "");
    mpv_win.end();
    mpv_win.set_color(Color::Black);

    win.end();
    win.show();
    win.make_resizable(true);

    let handle = mpv_win.raw_handle();
    std::process::Command::new("mpv")
        .args(&[&format!("--wid={}", handle as u64), "../libvlc/video.mp4"])
        .spawn()
        .unwrap();

    app.run().unwrap();
}
