use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let file = File::create("test_xml_sample.xml")?;
    let mut books = String::new();
    for _ in 0..2000000 {
        let title: String = (0..16).map(|_| rng.gen_range(b'A'..b'Z') as char).collect();
        let first_name: String = (0..8).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
        let last_name: String = (0..12).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
        let alive = if rng.gen_bool(0.5) { "yes" } else { "no" };
        books += &format!(
            "<book><title>{}</title><author><first_name>{}</first_name><last_name>{}</last_name><alive>{}</alive></author></book>",
            title, first_name, last_name, alive
        );
    }
    let content = format!("<books>{}</books>", books);
    let mut writer = LineWriter::new(file);
    writer.write_all(content.as_bytes())?;
    Ok(())
}
