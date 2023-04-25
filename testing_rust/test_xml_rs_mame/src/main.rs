extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

/*
   gi_game_info_id uuid NOT NULL,
   gi_game_info_system_id uuid,
   gi_game_info_short_name text COLLATE pg_catalog."default",
   gi_game_info_name text COLLATE pg_catalog."default",
   gi_game_info_json jsonb,
   gi_game_info_localimage jsonb,
   gi_game_info_sha1 text COLLATE pg_catalog."default",
   gi_game_info_blake3 text COLLATE pg_catalog."default",
*/

pub struct MAMEGameInfo {
    short_name: String,
    name: String,
    // json: Json,
    // image_json: Json,
}

fn main() {
    let file = File::open("C:/Users/qgranfor/Downloads/mame0241.xml").unwrap();
    //let file = File::open("/home/spoot/Downloads/mame0241lx/mame0241.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for xml_event in parser {
        match xml_event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
                ..
            }) => {
                println!(
                    "{}+{} {:?} {:?}",
                    indent(depth),
                    name,
                    attributes,
                    namespace
                );
                depth += 1;
            }
            Ok(XmlEvent::Characters(text)) => {
                println!("{}", text); // or something else
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(xml_event) => {
                println!("Error: {}", xml_event);
                break;
            }
            _ => {}
        }
    }
}
