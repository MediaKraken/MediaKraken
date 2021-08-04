use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(800, 480);
    //let mut frame = Frame::default().size_of(&wind);

    // left side buttons
    let mut button_in_progress = Button::new(0, 0, 133, 96, "In Progress");
    let mut image_in_progress = SharedImage::load("../../docker/core/mkwebapp/static/image/earth.png")?;
    image_in_progress.scale(133, 96, true, true);
    button_in_progress.set_image(Some(image_in_progress));
    button_in_progress.redraw();
    let mut button_new = Button::new(0, 96, 133, 96, "New");
    let mut button_movie = Button::new(0, 192, 133, 96, "Movie");
    let mut button_tv = Button::new(0, 288, 133, 96, "TV");
    let mut button_game = Button::new(0, 384, 133, 96, "Games");

    // top middle button
    let mut button_demo = Button::new(133, 0, 532, 384, "Demo");

    // bottom middle buttons
    let mut button_music = Button::new(133, 384, 133, 96, "Music");
    let mut button_live_tv = Button::new(266, 384, 133, 96, "Live TV");
    let mut button_home_video = Button::new(399, 384, 133, 96, "Home Video");
    let mut button_internet = Button::new(532, 384, 133, 96, "Internet");

    // right side buttons
    let mut button_music_video = Button::new(667, 0, 133, 96, "Music Video");
    let mut button_pictures = Button::new(667, 96, 133, 96, "Pictures");
    let mut button_radio = Button::new(667, 192, 133, 96, "Radio");
    let mut button_books = Button::new(667, 288, 133, 96, "Books");
    let mut button_settings = Button::new(667, 384, 133, 96, "Settings");

    // let mut image_new = SharedImage::load("../../docker/core/mkwebapp/static/image/earth.png")?;
    // image_new.scale(133, 96, true, true);
    //
    // frame.set_image(Some(image_new));

    // // To remove an image
    // frame.set_image(None::<SharedImage>);

    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
