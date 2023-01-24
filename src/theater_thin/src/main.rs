#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use fltk::{enums::Color, prelude::*, *};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::Arc;
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[path = "../../mk_lib_network/src/mk_lib_network_mediakraken.rs"]
mod mk_lib_network_mediakraken;

fn main() {
    let server_list = mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();
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
