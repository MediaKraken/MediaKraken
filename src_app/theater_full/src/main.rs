use fltk::{
    app,
    button::Button,
    dialog,
    group::Flex,
    image::PngImage,
    prelude::*,
    window::Window,
};
use fltk_theme::{SchemeType, WidgetScheme};
use std::error::Error;

const WINDOW_W: i32 = 800;
const WINDOW_H: i32 = 480;
const SIDE_COL_W: i32 = WINDOW_W / 6;
const BOTTOM_ROW_H: i32 = WINDOW_H / 5;

macro_rules! asset {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../../docker/core/mkwebaxum/static/image/",
            $name
        ))
    };
}

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

fn make_image_button(label: &str, image_bytes: &[u8]) -> Result<Button, Box<dyn Error>> {
    let mut button = Button::default().with_label(label);
    set_button_image(&mut button, image_bytes, SIDE_COL_W, BOTTOM_ROW_H)?;
    Ok(button)
}

fn run() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let _server = runtime.block_on(
        mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server(),
    );

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    WidgetScheme::new(SchemeType::Fluent).apply();

    let mut window_menu = Window::default()
        .with_size(WINDOW_W, WINDOW_H)
        .with_label("MediaKraken Theater");

    let mut outer = Flex::default_fill().row();
    outer.set_margin(0);
    outer.set_pad(0);

    let mut left_col = Flex::default().column();
    left_col.set_pad(0);
    let _button_in_progress = make_image_button("In Progress", asset!("rectangles_black.png"))?;
    let _button_new = make_image_button("New", asset!("new.png"))?;
    let _button_movie = make_image_button("Movie", asset!("movie_ticket.png"))?;
    let _button_tv = make_image_button("TV", asset!("television.png"))?;
    let _button_game = make_image_button("Games", asset!("vid_game.png"))?;
    left_col.end();

    let mut center_col = Flex::default().column();
    center_col.set_pad(0);
    let _button_demo = make_image_button("Demo", asset!("theater.png"))?;
    let mut bottom_row = Flex::default().row();
    bottom_row.set_pad(0);
    let _button_music = make_image_button("Music", asset!("headphone.png"))?;
    let _button_live_tv = make_image_button("Live TV", asset!("television_live.png"))?;
    let _button_home_video = make_image_button("Home Video", asset!("vid_camera.png"))?;
    let _button_internet = make_image_button("Internet", asset!("earth.png"))?;
    bottom_row.end();
    center_col.fixed(&bottom_row, BOTTOM_ROW_H);
    center_col.end();

    let mut right_col = Flex::default().column();
    right_col.set_pad(0);
    let _button_music_video = make_image_button(
        "Music Video",
        asset!("listening-music-video-clip-with-auricular.png"),
    )?;
    let _button_pictures = make_image_button("Pictures", asset!("photo.png"))?;
    let _button_radio = make_image_button("Radio", asset!("radio.png"))?;
    let _button_books = make_image_button("Books", asset!("books.png"))?;
    let mut button_settings = make_image_button("Settings", asset!("settings.png"))?;
    right_col.end();

    outer.fixed(&left_col, SIDE_COL_W);
    outer.fixed(&right_col, SIDE_COL_W);
    outer.end();

    window_menu.end();
    window_menu.make_resizable(true);
    window_menu.fullscreen(true);

    let mut window_settings = Window::default()
        .with_size(WINDOW_W, WINDOW_H)
        .with_label("MediaKraken Theater - Settings");
    let mut settings_flex = Flex::default_fill().column();
    settings_flex.set_margin(0);
    settings_flex.set_pad(0);
    let settings_body = Flex::default().row();
    settings_body.end();
    let mut settings_footer = Flex::default().row();
    settings_footer.set_pad(0);
    let footer_spacer = Flex::default().row();
    footer_spacer.end();
    let mut button_settings_back = make_image_button("Back", asset!("navigation/return.png"))?;
    settings_footer.fixed(&button_settings_back, SIDE_COL_W);
    settings_footer.end();
    settings_flex.fixed(&settings_footer, BOTTOM_ROW_H);
    settings_flex.end();
    window_settings.end();
    window_settings.make_resizable(true);
    window_settings.fullscreen(true);
    window_settings.hide();

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

    window_menu.show();

    app.run()?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("theater_full fatal error: {error}");
        dialog::alert_default(&format!("Fatal error: {error}"));
        std::process::exit(1);
    }
}
