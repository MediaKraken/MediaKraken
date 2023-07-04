use std::env;
use std::process::{Command, Stdio};
use serde_json::{Result, Value};

#[tokio::main]
async fn main() {
    let json_message = r#"{"Data": "06_28_2023"}"#;
    let v: Value = serde_json::from_str(json_message).unwrap();
    let format_data = format!(
        "http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz",
        v["Data"].as_str().unwrap()
    ).replace("\"", "");
    println!("{}", format_data);

    // let table_name = "_seq";
    // let sub = &table_name[table_name.len() - 4..].to_string();
    // println!("{}", sub);
    // if sub != "mm_" {
    //     println!("here")
    // }

    // let what = &format!(
    //     "\"\\copy {} from '/mediakraken/mbdump/{}';\"",
    //     "release", "release"
    // );
    // println!("{}", what);
    // env::set_var("PGPASSWORD", "password");
    // let output = Command::new("psql")
    //     .args([
    //         // "-h",
    //         // "mkstack_database",
    //         "-d",
    //         "postgres",
    //         "-U",
    //         "postgres",
    //         "-c",
    //         &format!(
    //             "\\copy {} from '/mediakraken/mbdump/{}';",
    //             "release", "release"
    //         ),
    //     ])
    //     .output()
    //     .expect("failed to execute process");
    // let stdout = String::from_utf8_lossy(&output.stdout);
    // println!("Output: {:?}", stdout);
    // let stderr = String::from_utf8_lossy(&output.stderr);
    // println!("Output Error: {:?}", stderr);
}
