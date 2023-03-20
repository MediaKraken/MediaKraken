use substring::Substring;
use std::str;
use std::error::Error;

pub async fn provider_televisiontunes_theme_fetch(tv_show_name: String)
                                                  -> Result<bool, Box<dyn std::error::Error>>{
    let base_url = "https://www.televisiontunes.com/".to_string();
    let show_url = format!("{}{}", base_url, tv_show_name.replace(" ", "_"));
    let response = reqwest::get(show_url).await?;
    println!("here");
    if response.status().is_success() {
        println!("here2");
        let content = response.bytes().await?;
        let content_string = str::from_utf8(&content).unwrap().to_string();
        let dl_position = content_string.find("href=\"/song/download/").unwrap();
        println!("{:?}", dl_position);
        let data_content = content_string.substring(dl_position + 21, dl_position + 50);
        let dl_end_position = data_content.find("\"").unwrap();
        println!("{:?}", data_content.substring(0, dl_end_position));
        let dl_url = format!("{}{}{}", base_url, "song/download/",
                             data_content.substring(0, dl_end_position));
        println!("{}", dl_url);
        Ok(true)
    }
    else {
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tv_show_name = "Sopranos".to_string();
    provider_televisiontunes_theme_fetch(tv_show_name).await.unwrap();
    Ok(())
}