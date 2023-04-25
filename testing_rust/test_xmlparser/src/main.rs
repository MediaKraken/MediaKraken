extern crate xmlparser as xml;

use std::fs;
use std::io::Read;

fn main() {
    let text = load_file();
    if let Err(e) = parse(&text) {
        println!("Error: {}.", e);
    }
}

fn parse(text: &str) -> Result<(), xml::Error> {
    for token in xml::Tokenizer::from(text) {
        println!("{:?}", token?);
    }
    Ok(())
}

fn load_file() -> String {
    let mut file = fs::File::open("/home/spoot/Downloads/mame0241lx/mame0241.xml").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
