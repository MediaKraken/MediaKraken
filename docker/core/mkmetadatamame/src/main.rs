use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::RSplit;
use std::{fs, io};
use tokio::sync::Notify;

// https://www.progettosnaps.net/download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats_236.7z

// technically arcade games are "systems"....
// they just don"t have @isdevice = "yes" like mess hardware does

// However, mame games are still being put as "games" and not systems
// to ease search and other filters by game/system

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();
    let option_config_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkmetadatamame")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmetadatamame", &rabbit_channel)
            .await
            .unwrap();

    println!("Here I am");
    // let local = tokio::task::LocalSet::new();
    // local.run_until(async move {
    println!("Here I am 2");
    //tokio::task::spawn_local(async move {
    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                println!("Here I am 3");
                // let _json_message: Value =
                //     serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                // create mame game list
                let file_name = format!(
                    "/mediakraken/emulation/mame0{}lx.zip",
                    option_config_json["MAME"]["Version"]
                );
                println!("File: {}", file_name);
                // only do the parse/import if not processed before
                if !Path::new(&file_name).exists() {
                    println!("File dl");
                    // https://github.com/mamedev/mame/releases/download/mame0256/mame0256lx.zip
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                            format!(
                                "https://github.com/mamedev/mame/releases/download/mame0{}/mame0{}lx.zip",
                                option_config_json["MAME"]["Version"], option_config_json["MAME"]["Version"]
                            ),
                            &file_name,
                        )
                        .await
                        .unwrap();
                    println!("File dl 2");
                    let unzip_file_name = format!(
                        "/mediakraken/emulation/mame0{}.xml",
                        option_config_json["MAME"]["Version"]
                    );
                    if !Path::new(&unzip_file_name).exists() {
                        mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                            &file_name,
                            false,
                            "/mediakraken/emulation/",
                        )
                        .await
                        .unwrap();
                        let file = File::open(&unzip_file_name).unwrap();
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
                                let json_data =
                                    xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                                // name is short name
                                // description is long name
                                mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                                        &sqlx_pool,
                                        uuid::Uuid::nil(),
                                        json_data["machine"]["name"].to_string(),
                                        json_data["machine"]["description"].to_string(),
                                        json_data,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                xml_data.push_str(xml_line);
                            }
                        }
                    }
                }

                println!("Here I am 55");
                // load games from hash files
                let file_name = format!(
                    "/mediakraken/emulation/mame0{}.zip",
                    option_config_json["MAME"]["Version"]
                );
                // only do the parse/import if not processed before
                if !Path::new(&file_name).exists() {
                    // https://github.com/mamedev/mame/archive/refs/tags/mame0256.zip
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        format!(
                            "https://github.com/mamedev/mame/archive/refs/tags/mame0{}.zip",
                            option_config_json["MAME"]["Version"]
                        ),
                        &file_name,
                    )
                    .await
                    .unwrap();
                    mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                        &file_name,
                        false,
                        &"/mediakraken/emulation/",
                    )
                    .await
                    .unwrap();

                    let entries = fs::read_dir(format!(
                        "/mediakraken/emulation/mame-mame0{}/hash",
                        option_config_json["MAME"]["Version"]
                    ))
                    .unwrap()
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()
                    .unwrap();
                    for hash_file_path in entries {
                        let ext = Path::new(&hash_file_path)
                            .extension()
                            .unwrap_or(&std::ffi::OsStr::new("no_extension"));
                        if ext == "xml" {
                            let file = File::open(&hash_file_path).unwrap();
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
                                // fetch sytem id from /softwarelist/name
                                if xml_line.starts_with("<softwarelist") == true {
                                    println!("xml_line: {:?}", xml_line);
                                    let system_string_split: Vec<&str> =
                                        xml_line.split("\"").collect();
                                    println!("split: {:?}", system_string_split);
                                    let system_counter = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_game_count_by_short_name(&sqlx_pool, &system_string_split[1].to_string()).await.unwrap();
                                    if system_counter == 0 {
                                        game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool, system_string_split[1].to_string(), String::new(), json!({})).await.unwrap();
                                    } else {
                                        game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool, &system_string_split[1].to_string()).await.unwrap();
                                    }
                                } else if xml_line.starts_with("<software") == true {
                                    xml_data = xml_line.to_string();
                                } else if xml_line.starts_with("</software") == true {
                                    xml_data.push_str(xml_line);
                                    let json_data =
                                        xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                                    // name is short name
                                    // description is long name
                                    mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                                        &sqlx_pool,
                                        game_system_uuid,
                                        json_data["software"]["name"].to_string(),
                                        json_data["software"]["description"].to_string(),
                                        json_data,
                                    )
                                    .await.unwrap();
                                } else {
                                    xml_data.push_str(xml_line);
                                }
                            }
                        }
                    }
                }

                println!("Here I am 33");
                // update mame game descriptions from history dat
                let file_name = format!(
                    "/mediakraken/emulation/historyxml{}.zip",
                    option_config_json["MAME"]["Version"]
                );
                // only do the parse/import if not processed before
                if !Path::new(&file_name).exists() {
                    // https://www.arcade-history.com/dats/history256b.zip
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        format!(
                            "https://www.arcade-history.com/dats/history{}b.zip",
                            option_config_json["MAME"]["Version"]
                        ),
                        &file_name,
                    )
                    .await
                    .unwrap();
                    mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                        &file_name,
                        false,
                        &"/mediakraken/emulation/",
                    )
                    .await
                    .unwrap();

                    let file = File::open(&"/mediakraken/emulation/history.xml").unwrap();
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
                            let json_data =
                                xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                            let mut game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool, &json_data["entry"]["software"]["item"]["list"].to_string()).await.unwrap();
                            if game_system_uuid == uuid::Uuid::nil() {
                                game_system_uuid = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool, json_data["entry"]["software"]["item"]["list"].to_string(), String::new(), json!({})).await.unwrap();
                            }
                            mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                                    &sqlx_pool,
                                    game_system_uuid,
                                    json_data["entry"]["software"]["item"]["name"].to_string(),
                                    json_data["entry"]["text"].to_string(),
                                    json_data,
                                )
                                .await
                                .unwrap();
                        } else {
                            xml_data.push_str(xml_line);
                        }
                    }
                }

                /*
                [Category]
                005=Maze / Shooter Small
                09825_67907=System / Device
                100lions=Slot Machine / Video Slot
                100lionsa=Slot Machine / Video Slot
                10yard=Sports / Football
                10yard85=Sports / Football
                10yardj=Sports / Football
                110dance=Handheld / Plug n' Play TV Game / Music
                */

                println!("Here I am 77");
                // read the category file and create dict/list for it
                let file_name = format!(
                    "/mediakraken/emulation/pS_CatVer_{}.zip",
                    option_config_json["MAME"]["Version"]
                );
                // only do the parse/import if not processed before
                if !Path::new(&file_name).exists() {
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                            format!(
                                "https://www.progettosnaps.net/download/?tipo=catver&file=pS_CatVer_{}.zip",
                                option_config_json["MAME"]["Version"]
                            ),
                            &file_name,
                        )
                        .await
                        .unwrap();
                    mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                        &file_name,
                        false,
                        &"/mediakraken/emulation/",
                    )
                    .await
                    .unwrap();
                    let file = File::open(&"/mediakraken/emulation/catver.ini").unwrap();
                    let reader = BufReader::new(file);
                    let mut category_found = false;
                    for line in reader.lines() {
                        let xml_line = &line.unwrap().trim().to_string();
                        if xml_line.len() > 1 {
                            // this is to skip blank lines
                            if category_found == true {
                                // everything here on out is a game / cat
                                // TODO
                                //  result_value = db_connection.db_meta_game_category_by_name(category)
                                //  if result_value == None:
                                //        result_value = db_connection.db_meta_game_category_add(category)
                                //        cat_dictionary[line.strip()] = result_value
                                //
                                // TODO
                                // grab all system null in db as those are mame
                                //     for sql_row in db_connection.db_media_mame_game_list():
                                //         db_connection.db_media_game_category_update(cat_dictionary[sql_row["gi_short_name"]],
                                //                                                     sql_row["gi_id"])
                                //
                                // TODO
                                // grab all the non parent roms that aren't set
                                //     for sql_row in db_connection.db_media_game_clone_list():
                                //         for sql_cat_row in db_connection.db_media_game_category_by_name(sql_row["gi_cloneof"]):
                                //             db_connection.db_media_game_category_update(sql_cat_row["gi_gc_category"],
                                //                                                         sql_row["gi_id"])
                                //
                            }
                            if xml_line.starts_with("[Category]") == true {
                                category_found = true;
                            }
                        }
                    }
                }

                println!("Here I am 99");
                // update mess system description
                let file_name = format!(
                    "/mediakraken/emulation/pS_messinfo_{}.zip",
                    option_config_json["MAME"]["Version"]
                );
                // only do the parse/import if not processed before
                if !Path::new(&file_name).exists() {
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                            format!("https://www.progettosnaps.net/download/?tipo=messinfo&file=pS_messinfo_{}.zip", option_config_json["MAME"]["Version"]), &file_name)
                        .await
                        .unwrap();
                    mk_lib_compression::mk_lib_compression::mk_decompress_zip(
                        &file_name,
                        false,
                        &"/mediakraken/emulation/",
                    )
                    .await
                    .unwrap();
                    let file = File::open(&"/mediakraken/emulation/messinfo.dat").unwrap();
                    let mut reader = BufReader::new(file);
                    let mut dat_line = String::new();
                    let mut start_system_read = false;
                    let mut skip_next_line = false;
                    let mut long_name_next = false;
                    let mut desc_next = false;
                    let mut wip_in_progress = false;
                    let mut romset_in_progress = false;
                    // store args to sql
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

                    let mut sql_string = String::new();

                    loop {
                        match reader.read_line(&mut dat_line) {
                            Ok(0) => break,
                            Ok(_) => {
                                if skip_next_line {
                                    skip_next_line = false;
                                } else {
                                    if dat_line.contains("DRIVERS INFO") {
                                        // stop at drivers
                                        break;
                                    }
                                    dat_line = dat_line.replace("    ", "");
                                    if dat_line.starts_with("#")
                                        || dat_line.len() < 4
                                        || dat_line.starts_with("$mame")
                                    {
                                        // skip comments and blank lines
                                        if dat_line.starts_with("$mame") {
                                            skip_next_line = true;
                                            long_name_next = true;
                                        }
                                    } else if dat_line.starts_with("$info") {
                                        // found so begin start system read
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
                                            // TODO
                                            // try:
                                            //     sys_longname, sys_manufacturer, sys_year = dat_line.split(",");
                                            // except:
                                            //     sys_longname, msys_manufacturer, sys_year = dat_line.rsplit(",", 2);
                                            long_name_next = false;
                                            desc_next = true;
                                        }
                                        if dat_line.starts_with("$end") {
                                            // end of system info so store system into db
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
                                        // upsert the system
                                        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool,
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
                                            );
                                        sys_wip = String::new();
                                        sys_romset = String::new();
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                println!("ACK!");
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });
    //}).await;

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
