use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::Arc;
use stdext::function_name;

fn main() -> Result<(), Box<dyn Error>> {
    let server_list = mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window_main = Window::default().with_size(800, 480); // pi 7" screen default
    let mut window_menu = Window::default().with_size(800, 480);
    // window_menu - left side buttons
    let mut button_in_progress = Button::new(0, 0, 133, 96, "In Progress");
    let mut image =
        SharedImage::load("../../docker/core/mkwebaxum/static/image/rectangles_black.png")?;
    image.scale(133, 96, true, true);
    button_in_progress.set_image(Some(image));
    let mut button_new = Button::new(0, 96, 133, 96, "New");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/new.png")?;
    image.scale(133, 96, true, true);
    button_new.set_image(Some(image));
    let mut button_movie = Button::new(0, 192, 133, 96, "Movie");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/movie_ticket.png")?;
    image.scale(133, 96, true, true);
    button_movie.set_image(Some(image));
    let mut button_tv = Button::new(0, 288, 133, 96, "TV");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/television.png")?;
    image.scale(133, 96, true, true);
    button_tv.set_image(Some(image));
    let mut button_game = Button::new(0, 384, 133, 96, "Games");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/vid_game.png")?;
    image.scale(133, 96, true, true);
    button_game.set_image(Some(image));
    // window_menu - top middle button
    let mut button_demo = Button::new(133, 0, 532, 384, "Demo");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/theater.png")?;
    image.scale(532, 384, true, true);
    button_demo.set_image(Some(image));
    // window_menu - bottom middle buttons
    let mut button_music = Button::new(133, 384, 133, 96, "Music");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/headphone.png")?;
    image.scale(133, 96, true, true);
    button_music.set_image(Some(image));
    let mut button_live_tv = Button::new(266, 384, 133, 96, "Live TV");
    let mut image =
        SharedImage::load("../../docker/core/mkwebaxum/static/image/television_live.png")?;
    image.scale(133, 96, true, true);
    button_live_tv.set_image(Some(image));
    let mut button_home_video = Button::new(399, 384, 133, 96, "Home Video");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/vid_camera.png")?;
    image.scale(133, 96, true, true);
    button_home_video.set_image(Some(image));
    let mut button_internet = Button::new(532, 384, 133, 96, "Internet");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/earth.png")?;
    image.scale(133, 96, true, true);
    button_internet.set_image(Some(image));
    // window_menu - right side buttons
    let mut button_music_video = Button::new(666, 0, 133, 96, "Music Video");
    let mut image = SharedImage::load(
        "../../docker/core/mkwebaxum/static/image/listening-music-video-clip-with-auricular.png",
    )?;
    image.scale(133, 96, true, true);
    button_music_video.set_image(Some(image));
    let mut button_pictures = Button::new(666, 96, 133, 96, "Pictures");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/photo.png")?;
    image.scale(133, 96, true, true);
    button_pictures.set_image(Some(image));
    let mut button_radio = Button::new(666, 192, 133, 96, "Radio");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/radio.png")?;
    image.scale(133, 96, true, true);
    button_radio.set_image(Some(image));
    let mut button_books = Button::new(666, 288, 133, 96, "Books");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/books.png")?;
    image.scale(133, 96, true, true);
    button_books.set_image(Some(image));
    let mut button_settings = Button::new(666, 384, 133, 96, "Settings");
    let mut image = SharedImage::load("../../docker/core/mkwebaxum/static/image/settings.png")?;
    image.scale(133, 96, true, true);
    button_settings.set_image(Some(image));
    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);

    let mut window_settings = Window::default().with_size(800, 480);
    let mut button_settings_back = Button::new(666, 384, 133, 96, "Back");
    let mut image =
        SharedImage::load("../../docker/core/mkwebaxum/static/image/navigation/return.png")?;
    image.scale(133, 96, true, true);
    button_settings_back.set_image(Some(image));
    window_settings.end();
    window_settings.make_resizable(true);
    window_settings.fullscreen(true);
    window_settings.hide();

    let mut window_media = Window::default().with_size(800, 480);

    window_media.end();
    window_media.make_resizable(true);
    window_media.fullscreen(true);
    window_media.hide();

    // close up the add to main window and setup current page
    window_main.end();
    window_main.make_resizable(true);
    window_main.fullscreen(true);
    window_main.show();
    window_menu.make_current();

    // // main button
    // button_settings.set_callback( |_b| {
    //     window_menu.hide();
    //     window_settings.show();
    // });
    //
    // // media list page
    //
    // // settings page
    // button_settings_back.set_callback(move |bb| {
    //     window_menu.show();
    //     window_settings.hide();
    // });

    app.run()?;
    Ok(())
}
