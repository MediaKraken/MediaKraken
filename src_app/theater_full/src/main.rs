use fltk::{app, button::Button, image::PngImage, prelude::*, window::Window};
use fltk_theme::{SchemeType, WidgetScheme};
use mk_lib_network;
use std::error::Error;

fn set_button_image(
    button: &mut Button,
    image_bytes: &[u8],
    width: i32,
    height: i32,
) -> Result<(), Box<dyn Error>> {
    let mut image = PngImage::from_data(image_bytes)?;
    image.scale(width, height, true, true);
    button.set_image(Some(image));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // load images
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
    let bytes_image_listening_music_video_clip_with_auricular = include_bytes!(
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
    let mut window_main = Window::default().with_size(800, 480); // pi 7" screen default
    let mut window_menu = Window::default().with_size(800, 480);
    // window_menu - left side buttons
    let mut button_in_progress = Button::new(0, 0, 133, 96, "In Progress");
    set_button_image(&mut button_in_progress, bytes_image_rectangle, 133, 96)?;
    let mut button_new = Button::new(0, 96, 133, 96, "New");
    set_button_image(&mut button_new, bytes_image_new, 133, 96)?;
    let mut button_movie = Button::new(0, 192, 133, 96, "Movie");
    set_button_image(&mut button_movie, bytes_image_movie_ticket, 133, 96)?;
    let mut button_tv = Button::new(0, 288, 133, 96, "TV");
    set_button_image(&mut button_tv, bytes_image_television, 133, 96)?;
    let mut button_game = Button::new(0, 384, 133, 96, "Games");
    set_button_image(&mut button_game, bytes_image_vid_game, 133, 96)?;
    // window_menu - top middle button
    let mut button_demo = Button::new(133, 0, 532, 384, "Demo");
    set_button_image(&mut button_demo, bytes_image_theater, 532, 384)?;
    // window_menu - bottom middle buttons
    let mut button_music = Button::new(133, 384, 133, 96, "Music");
    set_button_image(&mut button_music, bytes_image_headphone, 133, 96)?;
    let mut button_live_tv = Button::new(266, 384, 133, 96, "Live TV");
    set_button_image(&mut button_live_tv, bytes_image_television_live, 133, 96)?;
    let mut button_home_video = Button::new(399, 384, 133, 96, "Home Video");
    set_button_image(&mut button_home_video, bytes_image_vid_camera, 133, 96)?;
    let mut button_internet = Button::new(532, 384, 133, 96, "Internet");
    set_button_image(&mut button_internet, bytes_image_earth, 133, 96)?;
    // window_menu - right side buttons
    let mut button_music_video = Button::new(666, 0, 133, 96, "Music Video");
    set_button_image(
        &mut button_music_video,
        bytes_image_listening_music_video_clip_with_auricular,
        133,
        96,
    )?;
    let mut button_pictures = Button::new(666, 96, 133, 96, "Pictures");
    set_button_image(&mut button_pictures, bytes_image_photo, 133, 96)?;
    let mut button_radio = Button::new(666, 192, 133, 96, "Radio");
    set_button_image(&mut button_radio, bytes_image_radio, 133, 96)?;
    let mut button_books = Button::new(666, 288, 133, 96, "Books");
    set_button_image(&mut button_books, bytes_image_books, 133, 96)?;
    let mut button_settings = Button::new(666, 384, 133, 96, "Settings");
    set_button_image(&mut button_settings, bytes_image_settings, 133, 96)?;
    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);

    let mut window_settings = Window::default().with_size(800, 480);
    let mut button_settings_back = Button::new(666, 384, 133, 96, "Back");
    set_button_image(&mut button_settings_back, bytes_image_return, 133, 96)?;
    window_settings.end();
    window_settings.make_resizable(true);
    window_settings.fullscreen(true);
    window_settings.hide();

    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();

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
