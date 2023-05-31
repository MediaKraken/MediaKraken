use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// killed
// fn main() {
//     // read an XML file into a string
//     // let mut xml_file = File::open("/home/spoot/Downloads/mame0241lx/mame0241.xml");
//     // let mut xml_contents = String::new();
//     let xml_contents = std::fs::read_to_string(
//         "/home/spoot/Downloads/mame0241lx/mame0241.xml").unwrap();
//     // convert the XML string into JSON with default config params
//     let json = xml_string_to_json(xml_contents, &Config::new_with_defaults());
//     println!("{:?}", json);
// }

fn main() -> io::Result<()> {
    //let file = File::open("C:/Users/qgranfor/Downloads/mame0241.xml")?;
    let file = File::open("/home/spoot/Downloads/mame0241lx/mame0241.xml")?;
    let reader = BufReader::new(file);
    let mut xml_data: String = "".to_string();
    let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
        .add_json_type_override("/machine/@name", JsonArray::Infer(JsonType::AlwaysString))
        .add_json_type_override("/year", JsonArray::Infer(JsonType::AlwaysString))
        .add_json_type_override("/manufacturer", JsonArray::Infer(JsonType::AlwaysString));
    for line in reader.lines() {
        let xml_line = &line.unwrap().trim().to_string();
        if xml_line.starts_with("<machine") == true {
            println!("here");
            xml_data = xml_line.to_string();
        } else if xml_line.starts_with("</machine") == true {
            xml_data += xml_line;
            println!("xml {}", xml_data);
            let json = xml_string_to_json(xml_data.to_string(), &conf);
            println!("json {:?}", json.unwrap());
            //println!("json {:?}", json.unwrap()["machine"]["@name"]);
        } else {
            xml_data += xml_line;
        }
    }
    Ok(())
}
