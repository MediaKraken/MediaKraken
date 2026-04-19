use fltk::{
    app, app::*, enums::*, frame::*, group::*, input::*, output::Output, prelude::*, window::*,
};
mod choice;
mod database;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Scanned,
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_instance = database::database_open()?;
    let mut total_codes: i64 = database::database_upc_known_count(&db_instance)?;
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut window_main = Window::default().with_size(800, 480); // pi 7" screen default

    let mut choice_media_type = choice::MyChoice::new(20, 20, 90, 30, None);
    choice_media_type.add_choices(&["Media", "CD", "Book", "Misc"]);
    choice_media_type.set_current_choice(0);
    choice_media_type.button().set_frame(FrameType::BorderBox);
    choice_media_type.frame().set_frame(FrameType::BorderBox);

    let mut out = Output::new(100, 200, 400, 120, "");
    out.set_text_size(36);
    out.set_value("Started");

    let mut container_upc_codes = Pack::new(300, 25, 150, 40, "UPC Codes");

    let mut frame_upc_known = Frame::default()
        .with_size(40, 20)
        .with_label(format!("Known: {}", total_codes).as_str());

    container_upc_codes.end();
    container_upc_codes.set_frame(FrameType::BorderFrame);
    container_upc_codes.set_color(Color::Black);
    container_upc_codes.set_type(PackType::Vertical);

    let (s, r) = app::channel::<Message>();
    let mut upc_input = IntInput::new(100, 100, 400, 120, "UPC");
    upc_input.set_trigger(CallbackTrigger::EnterKey);
    upc_input.set_callback(move |_| {
        s.send(Message::Scanned);
    });

    window_main.end();
    window_main.show();
    window_main.make_current();

    upc_input.take_focus();
    upc_input.set_visible_focus();

    while app.wait() {
        if let Some(Message::Scanned) = r.recv() {
            let scanned_input = upc_input.value();
            println!("{}:", scanned_input);

            let output_text = match scanned_input.parse::<i64>() {
                Ok(upc_value) => match choice_media_type.value() {
                    Some(media_type) => handle_scan(
                        &db_instance,
                        upc_value,
                        media_type,
                        &mut total_codes,
                        &mut frame_upc_known,
                    ),
                    None => "Select a media type".to_string(),
                },
                Err(_) => "Invalid UPC".to_string(),
            };

            out.set_value(&output_text);
            upc_input.set_value("");
            upc_input.take_focus();
            upc_input.set_visible_focus();
        }
    }
    Ok(())
}

fn handle_scan(
    db: &sqlite::Connection,
    upc_value: i64,
    media_type: i32,
    total_codes: &mut i64,
    frame_upc_known: &mut Frame,
) -> String {
    if let Err(err) = database::database_insert_logs(db, &format!("{} scanned", upc_value)) {
        eprintln!("log insert failed: {}", err);
    }

    match database::database_upc_insert(db, upc_value, media_type) {
        Ok(true) => {
            *total_codes += 1;
            frame_upc_known.set_label(&format!("Known: {}", total_codes));
            if let Err(err) = database::database_insert_logs(db, &format!("{} added", upc_value)) {
                eprintln!("log insert failed: {}", err);
            }
            format!("{} - Added", upc_value)
        }
        Ok(false) => {
            if let Err(err) =
                database::database_insert_logs(db, &format!("{} duplicate", upc_value))
            {
                eprintln!("log insert failed: {}", err);
            }
            format!("{} - DUPLICATE!", upc_value)
        }
        Err(err) => {
            eprintln!("upc insert failed: {}", err);
            "Database error".to_string()
        }
    }
}
