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

fn make_image_button(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    label: &str,
    image_bytes: &[u8],
) -> Result<Button, Box<dyn Error>> {
    let mut button = Button::new(x, y, w, h, label);
    set_button_image(&mut button, image_bytes, w, h)?;
    Ok(button)
}

fn main() -> Result<(), Box<dyn Error>> {
    // load images
    let bytes_image_rectangle =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/rectangles_black.png");
    let bytes_image_new =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/new.png");
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
    let bytes_image_earth =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/earth.png");
    let bytes_image_music_video = include_bytes!(
        "../../../docker/core/mkwebaxum/static/image/listening-music-video-clip-with-auricular.png"
    );
    let bytes_image_photo =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/photo.png");
    let bytes_image_radio =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/radio.png");
    let bytes_image_books =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/books.png");
    let bytes_image_settings =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/settings.png");
    let bytes_image_return =
        include_bytes!("../../../docker/core/mkwebaxum/static/image/navigation/return.png");

    let _server_list =
        mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();

    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();

    // main menu window
    let mut window_menu = Window::default().with_size(800, 480);

    // left column
    let _button_in_progress =
        make_image_button(0, 0, 133, 96, "In Progress", bytes_image_rectangle)?;
    let _button_new =
        make_image_button(0, 96, 133, 96, "New", bytes_image_new)?;
    let _button_movie =
        make_image_button(0, 192, 133, 96, "Movie", bytes_image_movie_ticket)?;
    let _button_tv =
        make_image_button(0, 288, 133, 96, "TV", bytes_image_television)?;
    let _button_game =
        make_image_button(0, 384, 133, 96, "Games", bytes_image_vid_game)?;

    // center
    let _button_demo =
        make_image_button(133, 0, 532, 384, "Demo", bytes_image_theater)?;
    let _button_music =
        make_image_button(133, 384, 133, 96, "Music", bytes_image_headphone)?;
    let _button_live_tv =
        make_image_button(266, 384, 133, 96, "Live TV", bytes_image_television_live)?;
    let _button_home_video =
        make_image_button(399, 384, 133, 96, "Home Video", bytes_image_vid_camera)?;
    let _button_internet =
        make_image_button(532, 384, 133, 96, "Internet", bytes_image_earth)?;

    // right column
    let _button_music_video =
        make_image_button(666, 0, 133, 96, "Music Video", bytes_image_music_video)?;
    let _button_pictures =
        make_image_button(666, 96, 133, 96, "Pictures", bytes_image_photo)?;
    let _button_radio =
        make_image_button(666, 192, 133, 96, "Radio", bytes_image_radio)?;
    let _button_books =
        make_image_button(666, 288, 133, 96, "Books", bytes_image_books)?;
    let mut button_settings =
        make_image_button(666, 384, 133, 96, "Settings", bytes_image_settings)?;

    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);

    // settings window
    let mut window_settings = Window::default().with_size(800, 480);
    let mut button_settings_back =
        make_image_button(666, 384, 133, 96, "Back", bytes_image_return)?;
    window_settings.end();
    window_settings.make_resizable(true);
    window_settings.fullscreen(true);
    window_settings.hide();

    // callbacks
    {
        let mut menu = window_menu.clone();
        let mut settings = window_settings.clone();
        button_settings.set_callback(move |_| {
            menu.hide();
            settings.show();
        });
    }

    {
        let mut menu = window_menu.clone();
        let mut settings = window_settings.clone();
        button_settings_back.set_callback(move |_| {
            settings.hide();
            menu.show();
        });
    }

    // show the real UI window
    window_menu.show();

    app.run()?;
    Ok(())
}