use std::env;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() {
    let table_name = "_seq";
    let sub = &table_name[table_name.len() - 4..].to_string();
    println!("{}", sub);
    if sub != "mm_" {
        println!("here")
    }
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
