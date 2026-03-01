use fltk::{app, prelude::*};
use fltk::{enums::Color, *};
use mk_lib_network;

fn main() {
    let _server_list =
        mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();
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
    if let Err(error) = std::process::Command::new("mpv")
        .arg(format!("--wid={}", handle as u64))
        .arg("../libvlc/video.mp4")
        .spawn()
    {
        eprintln!("Failed to launch mpv: {error}");
    }

    if let Err(error) = app.run() {
        eprintln!("Application exited with error: {error}");
    }
}
