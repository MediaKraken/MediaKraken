use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use quickxml_to_serde::{Config, JsonArray, JsonType, NullValue, xml_string_to_json};
use serde_json::Value;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::Path;
use std::{fs, io};
use tokio::sync::Notify;

async fn process_rabbit_message(
    msg: serde_json::Value,
    rabbit_channel: &amqprs::channel::Channel,
    sqlx_pool_rw: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    let json_message: Value = serde_json::from_value(msg)?;
    let file_name = format!(
        "/mediakraken/emulation/mame0{}lx.zip",
        json_message["Version"]
    );
    println!("File: {}", file_name);
    if !Path::new(&file_name).exists() {
        println!("File dl");
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
                format!(
                    "https://github.com/mamedev/mame/releases/download/mame0{}/mame0{}lx.zip",
                    json_message["Version"], json_message["Version"]
                ),
                &file_name,
            )
            .await?;
        println!("File dl 2");
        let unzip_file_name = format!(
            "/mediakraken/emulation/mame0{}.xml",
            json_message["Version"]
        );
        if !Path::new(&unzip_file_name).exists() {
            mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                &file_name,
                false,
                "/mediakraken/emulation/",
            )
            .await?;
            let file = File::open(&unzip_file_name)?;
            let reader = BufReader::new(file);
            let mut xml_data: String = "".to_owned();
            let conf =
                Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
                    .add_json_type_override(
                        "/machine/name",
                        JsonArray::Infer(JsonType::AlwaysString),
                    )
                    .add_json_type_override(
                        "/year",
                        JsonArray::Infer(JsonType::AlwaysString),
                    )
                    .add_json_type_override(
                        "/manufacturer",
                        JsonArray::Infer(JsonType::AlwaysString),
                    );
            for line in reader.lines() {
                let xml_line = &line.unwrap().trim().to_string();
                if xml_line.starts_with("<machine") == true {
                    xml_data = xml_line.to_string();
                } else if xml_line.starts_with("</machine") == true {
                    xml_data.push_str(xml_line);
                    let json_data = xml_string_to_json(xml_data.to_string(), &conf)?;
                    mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                            &sqlx_pool_rw,
                            uuid::Uuid::nil(),
                            json_data["machine"]["name"].to_string(),
                            json_data["machine"]["description"].to_string(),
                            json_data,
                        )
                        .await?;
                } else {
                    xml_data.push_str(xml_line);
                }
            }
        }
    }

    println!("Here I am 33");
    let file_name = format!(
        "/mediakraken/emulation/historyxml{}.zip",
        json_message["Version"]
    );
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
            format!(
                "https://www.arcade-history.com/dats/history{}b.zip",
                json_message["Version"]
            ),
            &file_name,
        )
        .await?;
        mk_lib_compression::mk_lib_compression::mk_decompress_zip(
            &file_name,
            false,
            &"/mediakraken/emulation/",
        )
        .await?;

        let file = File::open(&"/mediakraken/emulation/history.xml")?;
        let reader = BufReader::new(file);
        let mut xml_data: String = "".to_owned();
        let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
            .add_json_type_override(
                "/entry/software/item/name",
                JsonArray::Infer(JsonType::AlwaysString),
            );
        for line in reader.lines() {
            let xml_line = &line.unwrap().trim().to_string();
            if xml_line.starts_with("<entry") == true {
                xml_data = xml_line.to_string();
            } else if xml_line.starts_with("</entry") == true {
                xml_data.push_str(xml_line);
                let json_data = xml_string_to_json(xml_data.to_string(), &conf)?;
                let mut game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool_rw, &json_data["entry"]["software"]["item"]["list"].to_string()).await?;
                if game_system_uuid == uuid::Uuid::nil() {
                    game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool_rw, json_data["entry"]["software"]["item"]["list"].to_string(), String::new(), json!({})).await?;
                }
                mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                        &sqlx_pool_rw,
                        game_system_uuid,
                        json_data["entry"]["software"]["item"]["name"].to_string(),
                        json_data["entry"]["text"].to_string(),
                        json_data,
                    )
                    .await?;
            } else {
                xml_data.push_str(xml_line);
            }
        }
    }

    println!("Here I am 77");
    let file_name = format!(
        "/mediakraken/emulation/pS_CatVer_{}.zip",
        json_message["Version"]
    );
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
                format!(
                    "https://www.progettosnaps.net/download/?tipo=catver&file=pS_CatVer_{}.zip",
                    json_message["Version"]
                ),
                &file_name,
            )
            .await?;
        mk_lib_compression::mk_lib_compression::mk_decompress_zip(
            &file_name,
            false,
            &"/mediakraken/emulation/",
        )
        .await?;
        let file = File::open(&"/mediakraken/emulation/catver.ini")?;
        let reader = BufReader::new(file);
        let mut category_found = false;
        for line in reader.lines() {
            let xml_line = &line.unwrap().trim().to_string();
            if xml_line.len() > 1 {
                if category_found == true {
                    // TODO
                }
            }
            if xml_line.starts_with("[Category]") == true {
                category_found = true;
            }
        }
    }

    println!("Here I am 99");
    let file_name = format!(
        "/mediakraken/emulation/pS_messinfo_{}.zip",
        json_message["Version"]
    );
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
                format!("https://www.progettosnaps.net/download/?tipo=messinfo&file=pS_messinfo_{}.zip", json_message["Version"]), &file_name)
            .await?;
        mk_lib_compression::mk_lib_compression::mk_decompress_zip(
            &file_name,
            false,
            &"/mediakraken/emulation/",
        )
        .await?;
        let file = File::open(&"/mediakraken/emulation/messinfo.dat")?;
        let mut reader = BufReader::new(file);
        let mut dat_line = String::new();
        let mut start_system_read = false;
        let mut skip_next_line = false;
        let mut long_name_next = false;
        let mut desc_next = false;
        let mut wip_in_progress = false;
        let mut romset_in_progress = false;
        let mut sys_short_name = String::new();
        let mut sys_longname = String::new();
        let mut sys_manufacturer = String::new();
        let mut sys_year = String::new();
        let mut sys_desc = String::new();
        let mut sys_emulation = String::new();
        let mut sys_color = String::new();
        let mut sys_sound = String::new();
        let mut sys_graphics = String::new();
        let mut sys_save_state = false;
        let mut sys_wip = String::new();
        let mut sys_romset = String::new();

        loop {
            dat_line.clear();
            match reader.read_line(&mut dat_line) {
                Ok(0) => break,
                Ok(_) => {
                    if skip_next_line {
                        skip_next_line = false;
                    } else {
                        if dat_line.contains("DRIVERS INFO") {
                            break;
                        }
                        dat_line = dat_line.replace("    ", "");
                        if dat_line.starts_with("#")
                            || dat_line.len() < 4
                            || dat_line.starts_with("$mame")
                        {
                            if dat_line.starts_with("$mame") {
                                skip_next_line = true;
                                long_name_next = true;
                            }
                        } else if dat_line.starts_with("$info") {
                            start_system_read = true;
                            sys_short_name =
                                dat_line.split("=").nth(1).unwrap().to_string();
                        } else if dat_line.starts_with("Emulation:") {
                            sys_emulation =
                                dat_line.split(" ").nth(1).unwrap().to_string();
                        } else if dat_line.starts_with("Color:") {
                            sys_color = dat_line.split(" ").nth(1).unwrap().to_string();
                        } else if dat_line.starts_with("Sound:") {
                            sys_sound = dat_line.split(" ").nth(1).unwrap().to_string();
                        } else if dat_line.starts_with("Graphics:") {
                            sys_graphics =
                                dat_line.split(" ").nth(1).unwrap().to_string();
                        } else if dat_line.starts_with("Save State:") {
                            if dat_line
                                .rsplit(" ")
                                .last()
                                .unwrap()
                                .trim_end_matches('\n')
                                == "Supported"
                            {
                                sys_save_state = true;
                            } else {
                                sys_save_state = false;
                            }
                        } else if dat_line.starts_with("WIP:") {
                            wip_in_progress = true;
                        } else if dat_line.starts_with("Romset:") {
                            wip_in_progress = false;
                            romset_in_progress = true;
                        } else {
                            if wip_in_progress && dat_line.find("Romset:") != Some(0) {
                                sys_wip.push_str(dat_line.trim_end_matches('\n'));
                                sys_wip.push_str("<BR>");
                            }
                            if romset_in_progress && dat_line.find("$end") != Some(0) {
                                sys_romset.push_str(dat_line.trim_end_matches('\n'));
                                sys_romset.push_str("<BR>");
                            }
                            if desc_next {
                                sys_desc = dat_line.clone();
                                desc_next = false;
                            }
                            if long_name_next {
                                let mut split_items = dat_line.split(",");
                                if let Some(first) = split_items.next() {
                                    sys_longname = first.to_string();
                                    if let Some(second) = split_items.next() {
                                        sys_manufacturer = second.to_string();
                                        if let Some(third) = split_items.next() {
                                            sys_year = third.to_string();
                                        }
                                    }
                                }
                                long_name_next = false;
                                desc_next = true;
                            }
                            if dat_line.starts_with("$end") {
                                romset_in_progress = false;
                                if sys_desc.trim_end_matches('\n') == "..." {
                                    sys_desc = String::new();
                                } else {
                                    sys_desc =
                                        sys_desc.trim_end_matches('\n').to_string();
                                }
                                sys_emulation =
                                    sys_emulation.trim_end_matches('\n').to_string();
                                sys_color =
                                    sys_color.trim_end_matches('\n').to_string();
                                sys_sound =
                                    sys_sound.trim_end_matches('\n').to_string();
                                sys_graphics =
                                    sys_graphics.trim_end_matches('\n').to_string();
                            }
                            let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool_rw,
                                    sys_short_name.trim_end_matches('\n').to_string(),
                                    sys_longname.clone(),
                                    json!({
                                    "Desc": sys_desc,
                                    "Year": sys_year.trim_end_matches('\n'),
                                    "Manufacturer": sys_manufacturer,
                                    "Emulation": sys_emulation,
                                    "Color": sys_color,
                                    "Sound": sys_sound,
                                    "Graphics": sys_graphics,
                                    "Save State": sys_save_state})
                                ).await?;
                            sys_wip = String::new();
                            sys_romset = String::new();
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }

    println!("Here I am 55");
    let file_name = format!(
        "/mediakraken/emulation/mame0{}.zip",
        json_message["Version"]
    );
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
            format!(
                "https://github.com/mamedev/mame/archive/refs/tags/mame0{}.zip",
                json_message["Version"]
            ),
            &file_name,
        )
        .await?;
        mk_lib_compression::mk_lib_compression::mk_decompress_zip(
            &file_name,
            false,
            &"/mediakraken/emulation/",
        )
        .await?;

        let entries = fs::read_dir(format!(
            "/mediakraken/emulation/mame-mame0{}/hash",
            json_message["Version"]
        ))
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
        for hash_file_path in entries {
            let ext = Path::new(&hash_file_path)
                .extension()
                .unwrap_or(&std::ffi::OsStr::new("no_extension"));
            if ext == "xml" {
                let file = File::open(&hash_file_path)?;
                let reader = BufReader::new(file);
                let mut xml_data: String = "".to_owned();
                let conf =
                    Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
                        .add_json_type_override(
                            "/software/name",
                            JsonArray::Infer(JsonType::AlwaysString),
                        )
                        .add_json_type_override(
                            "/software/year",
                            JsonArray::Infer(JsonType::AlwaysString),
                        )
                        .add_json_type_override(
                            "/software/publisher",
                            JsonArray::Infer(JsonType::AlwaysString),
                        );
                let mut game_system_uuid = uuid::Uuid::nil();
                for line in reader.lines() {
                    let xml_line = &line.unwrap().trim().to_string();
                    if xml_line.starts_with("<softwarelist") == true {
                        println!("xml_line: {:?}", xml_line);
                        let system_string_split: Vec<&str> =
                            xml_line.split("\"").collect();
                        println!("split: {:?}", system_string_split);
                        let system_counter = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_game_count_by_short_name(&sqlx_pool_rw, &system_string_split[1].to_string()).await?;
                        if system_counter == 0 {
                            game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool_rw, system_string_split[1].to_string(), system_string_split[3].to_string(), json!({})).await?;
                        } else {
                            game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool_rw, &system_string_split[1].to_string()).await?;
                        }
                    } else if xml_line.starts_with("<software") == true {
                        xml_data = xml_line.to_string();
                    } else if xml_line.starts_with("</software") == true {
                        xml_data.push_str(xml_line);
                        let json_data =
                            xml_string_to_json(xml_data.to_string(), &conf)?;
                        mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                            &sqlx_pool_rw,
                            game_system_uuid,
                            json_data["software"]["name"].to_string(),
                            json_data["software"]["description"].to_string(),
                            json_data,
                        )
                        .await?;
                    } else {
                        xml_data.push_str(xml_line);
                    }
                }
            }
        }
    }

    println!("ACK!");
    Ok(())
}

// https://www.progettosnaps.net/download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats_236.7z

// technically arcade games are "systems"....
// they just don"t have @isdevice = "yes" like mess hardware does

// However, mame games are still being put as "games" and not systems
// to ease search and other filters by game/system

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmetadatamame").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmetadatamame", &rabbit_channel)
            .await?;

    let spawn_pool_rw = sqlx_pool_rw.clone();
    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let parsed: Value = serde_json::from_slice(&payload).unwrap_or_default();
                if let Err(e) = process_rabbit_message(parsed, &rabbit_channel, &spawn_pool_rw).await {
                    eprintln!("Processing failed: {}", e);
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.map(|d| d.delivery_tag()).unwrap_or(0),
                )
                .await;
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
