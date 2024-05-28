use fltk::{
    app, app::*, button::*, enums::*, frame::*, group::*, input::*, output::Output, prelude::*,
    window::*,
};
mod choice;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Scanned,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window_main = Window::default().with_size(800, 480); // pi 7" screen default

    let mut button_sync = Button::new(666, 384, 133, 96, "Sync");

    let mut choice_media_type = choice::MyChoice::new(20, 20, 90, 30, None);
    choice_media_type.add_choices(&["UHD", "BluRay", "DVD", "CD", "Book", "HDDVD", "LASERDISC", "GAME"]);
    choice_media_type.set_current_choice(0);
    choice_media_type.button().set_frame(FrameType::BorderBox);
    choice_media_type.frame().set_frame(FrameType::BorderBox);

    let mut out = Output::new(100, 200, 400, 120, "");
    out.set_text_size(36);
    out.set_value("Started");

    let mut container_upc_codes = Pack::new(300, 25, 150, 40, "UPC Codes");

    let mut frame_upc_known = Frame::default().with_size(40, 20).with_label(format!("Known: {}", total_codes).as_str());

    container_upc_codes.end();
    container_upc_codes.set_frame(FrameType::BorderFrame);
    container_upc_codes.set_color(Color::Black);
    container_upc_codes.set_type(PackType::Vertical);

    // setup the event
    let (s, r) = app::channel::<Message>();
    let mut upc_input = IntInput::new(100, 100, 400, 120, "UPC");
    upc_input.set_trigger(CallbackTrigger::EnterKey);
    upc_input.set_callback({
        move |_| {
            s.send(Message::Scanned);
        }
    });

    window_main.end();
    window_main.show();
    window_main.make_current();

    upc_input.take_focus();
    upc_input.set_visible_focus();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => {
                    println!("Increment");
                }
                Message::Scanned => {
                    println!("{}:", upc_input.value());

                    upc_input.set_value("");
                    upc_input.take_focus();
                    upc_input.set_visible_focus();
                }
            }
        }
    }
    Ok(())
}
