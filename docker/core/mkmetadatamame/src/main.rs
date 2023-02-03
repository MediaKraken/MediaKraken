#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use async_std::path::PathBuf;
use chrono::prelude::*;
use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use stdext::function_name;

// https://www.progettosnaps.net/download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats_236.7z

#[path = "mk_lib_compression.rs"]
mod mk_lib_compression;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;
#[path = "mk_lib_database_metadata_game_system.rs"]
mod mk_lib_database_metadata_game_system;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

// technically arcade games are "systems"....
// they just don"t have @isdevice = "yes" like mess hardware does

// However, mame games are still being put as "games" and not systems
// to ease search and other filters by game/system

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"})).await.unwrap();
    }

    // open the database
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(1).await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();
    let option_config_json: serde_json::Value =
        mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    // create mame game list
    let file_name = format!(
        "/mediakraken/emulation/mame0{}lx.zip",
        option_config_json["MAME"]["Version"]
    );
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://github.com/mamedev/mame/releases/download/mame0{}/mame0{}lx.zip",
                option_config_json["MAME"]["Version"], option_config_json["MAME"]["Version"]
            ),
            &file_name,
        )
        .await
        .unwrap();
        let unzip_file_name = format!(
            "/mediakraken/emulation/mame0{}.xml",
            option_config_json["MAME"]["Version"]
        );
        if !Path::new(&unzip_file_name).exists() {
            mk_lib_compression::mk_decompress_zip(&file_name, false, "/mediakraken/emulation/")
                .await.unwrap();
            let file = File::open(&unzip_file_name)?;
            let reader = BufReader::new(file);
            let mut xml_data: String = "".to_owned();
            let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
                .add_json_type_override("/machine/@name", JsonArray::Infer(JsonType::AlwaysString))
                .add_json_type_override("/year", JsonArray::Infer(JsonType::AlwaysString))
                .add_json_type_override("/manufacturer", JsonArray::Infer(JsonType::AlwaysString));
            for line in reader.lines() {
                let xml_line = &line.unwrap().trim().to_string();
                if xml_line.starts_with("<machine") == true {
                    xml_data = xml_line.to_string();
                } else if xml_line.starts_with("</machine") == true {
                    xml_data.push_str(xml_line);
                    let json_data = xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                    // name is short name
                    // description is long name
                    mk_lib_database_metadata_game::mk_lib_database_metadata_game_upsert(
                        &sqlx_pool,
                        uuid::Uuid::nil(),
                        json_data["machine"]["@name"].to_string(),
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

    // load games from hash files
    let file_name = format!(
        "/mediakraken/emulation/mame0{}.zip",
        option_config_json["MAME"]["Version"]
    );
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://github.com/mamedev/mame/archive/mame0{}.zip",
                option_config_json["MAME"]["Version"]
            ),
            &file_name,
        )
        .await
        .unwrap();
        mk_lib_compression::mk_decompress_zip(&file_name, false, &"/mediakraken/emulation/")
            .await.unwrap();
        for zippedfile in mk_lib_file::mk_directory_walk(format!(
            "/mediakraken/emulation/mame-mame0{}/hash",
            option_config_json["MAME"]["Version"]
        ))
        .await
        .unwrap()
        {
            //let file_name = Path::new(&zippedfile).file_stem().unwrap();
            let ext = Path::new(&zippedfile)
                .extension()
                .unwrap_or(&std::ffi::OsStr::new("no_extension"));
            if ext == ".xml" {
                let file = File::open(&zippedfile)?;
                let reader = BufReader::new(file);
                let mut xml_data: String = "".to_owned();
                let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
                    .add_json_type_override(
                        "/software/@name",
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
                    // fetch sytem id from /softwarelist/@name
                    if xml_line.starts_with("<softwarelist") == true {
                        let system_string_split: Vec<&str> = xml_line.split("\"").collect();
                        game_system_uuid = mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool, system_string_split[1].to_string()).await.unwrap();
                        if game_system_uuid == uuid::Uuid::nil() {
                            game_system_uuid = mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool, system_string_split[1].to_string(), String::new(), json!({})).await.unwrap();
                        }
                    } else if xml_line.starts_with("<software") == true {
                        xml_data = xml_line.to_string();
                    } else if xml_line.starts_with("</software") == true {
                        xml_data.push_str(xml_line);
                        let json_data = xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                        // name is short name
                        // description is long name
                        mk_lib_database_metadata_game::mk_lib_database_metadata_game_upsert(
                            &sqlx_pool,
                            game_system_uuid,
                            json_data["software"]["@name"].to_string(),
                            json_data["software"]["description"].to_string(),
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
    }

    // update mame game descriptions from history dat
    let file_name = format!(
        "/mediakraken/emulation/historyxml{}.zip",
        option_config_json["MAME"]["Version"]
    );
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://www.arcade-history.com/dats/historyxml{}.zip",
                option_config_json["MAME"]["Version"]
            ),
            &file_name,
        )
        .await
        .unwrap();
        mk_lib_compression::mk_decompress_zip(&file_name, false, &"/mediakraken/emulation/")
            .await.unwrap();

        let file = File::open(&format!(
            "/mediakraken/emulation/historyxml{}/historyxml{}.xml",
            option_config_json["MAME"]["Version"], option_config_json["MAME"]["Version"]
        ))?;
        let reader = BufReader::new(file);
        let mut xml_data: String = "".to_owned();
        let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
            .add_json_type_override(
                "/entry/software/item/@name",
                JsonArray::Infer(JsonType::AlwaysString),
            );
        for line in reader.lines() {
            let xml_line = &line.unwrap().trim().to_string();
            if xml_line.starts_with("<entry") == true {
                xml_data = xml_line.to_string();
            } else if xml_line.starts_with("</entry") == true {
                xml_data.push_str(xml_line);
                let json_data = xml_string_to_json(xml_data.to_string(), &conf).unwrap();
                let mut game_system_uuid = mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_guid_by_short_name(&sqlx_pool, json_data["entry"]["software"]["item"]["@list"].to_string()).await.unwrap();
                if game_system_uuid == uuid::Uuid::nil() {
                    game_system_uuid = mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_upsert(&sqlx_pool, json_data["entry"]["software"]["item"]["@list"].to_string(), String::new(), json!({})).await.unwrap();
                }
                mk_lib_database_metadata_game::mk_lib_database_metadata_game_upsert(
                    &sqlx_pool,
                    game_system_uuid,
                    json_data["entry"]["software"]["item"]["@name"].to_string(),
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

    // read the category file and create dict/list for it
    let file_name = format!(
        "/mediakraken/emulation/pS_CatVer_{}.zip",
        option_config_json["MAME"]["Version"]
    );
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://www.progettosnaps.net/download/?tipo=catver&file=pS_CatVer_{}.zip",
                option_config_json["MAME"]["Version"]
            ),
            &file_name,
        )
        .await
        .unwrap();
        mk_lib_compression::mk_decompress_zip(&file_name, false, &"/mediakraken/emulation/")
            .await.unwrap();
        let file = File::open(&format!(
            "/mediakraken/emulation/pS_CatVer_{}/catver.ini",
            option_config_json["MAME"]["Version"]
        ))?;
        let reader = BufReader::new(file);
        let mut category_found = false;
        for line in reader.lines() {
            let xml_line = &line.unwrap().trim().to_string();
            if xml_line.len() > 1 {
                if category_found == true {
                    // everything here on out is a game / cat
                }
                if xml_line.starts_with("[Category]") == true {
                    category_found = true;
                }
            }
            //  result_value = db_connection.db_meta_game_category_by_name(category)
            //  if result_value == None:
            //        result_value = db_connection.db_meta_game_category_add(category)
            //        cat_dictionary[line.strip()] = result_value
            //
            // grab all system null in db as those are mame
            //     for sql_row in db_connection.db_media_mame_game_list():
            //         db_connection.db_media_game_category_update(cat_dictionary[sql_row["gi_short_name"]],
            //                                                     sql_row["gi_id"])
            //
            // grab all the non parent roms that aren't set
            //     for sql_row in db_connection.db_media_game_clone_list():
            //         for sql_cat_row in db_connection.db_media_game_category_by_name(sql_row["gi_cloneof"]):
            //             db_connection.db_media_game_category_update(sql_cat_row["gi_gc_category"],
            //                                                         sql_row["gi_id"])
            //
        }
    }

    // update mess system description
    let file_name = format!(
        "/mediakraken/emulation/pS_messinfo_{}.zip",
        option_config_json["MAME"]["Version"]
    );
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://www.progettosnaps.net/download?tipo=messinfo&file=pS_messinfo_{}.zip",
                option_config_json["MAME"]["Version"]
            ),
            &file_name,
        )
        .await
        .unwrap();
        mk_lib_compression::mk_decompress_zip(&file_name, false, &"/mediakraken/emulation/")
            .await.unwrap();
        let file = File::open(&format!(
            "/mediakraken/emulation/pS_messinfo_{}/messinfo.dat",
            option_config_json["MAME"]["Version"]
        ))?;
        let reader = BufReader::new(file);

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
        let mut sys_year: i8 = 0;
        let mut sys_desc = String::new();
        let mut sys_emulation = String::new();
        let mut sys_color = String::new();
        let mut sys_sound = String::new();
        let mut sys_graphics = String::new();
        let mut sys_save_state = String::new();
        let mut sys_wip = String::new();
        let mut sys_romset = String::new();

        let mut sql_string = String::new();
        //     while 1:
        //         line = infile.readline()
        //         if not line:
        //             break
        //         if skip_next_line:
        //             skip_next_line = false
        //         else:
        //             if line.find("DRIVERS INFO") != -1:  // stop at drivers
        //                 break
        //             line = line.replace("    ", "")
        //             if line[0] == "#" or len(line) < 4 \
        //                     or line.find("$mame") == 0:  // skip comments and blank lines
        //                 if line.find("$mame") == 0:
        //                     skip_next_line = true
        //                     long_name_next = true
        //             else if line.find("$info") == 0:  // found so begin start system read
        //                 start_system_read = true
        //                 // load the short name
        //                 sys_short_name = line.split("=")[1]
        //             else if line.find("Emulation:") == 0:  // found so begin start system read
        //                 sys_emulation = line.split(" ")[1]
        //             else if line.find("Color:") == 0:  // found so begin start system read
        //                 sys_color = line.split(" ")[1]
        //             else if line.find("Sound:") == 0:  // found so begin start system read
        //                 sys_sound = line.split(" ")[1]
        //             else if line.find("Graphics:") == 0:  // found so begin start system read
        //                 sys_graphics = line.split(" ")[1]
        //             else if line.find("Save State:") == 0:  // found so begin start system read
        //                 if line.rsplit(" ", 1)[1][:-1] == "Supported":
        //                     sys_save_state = true
        //                 else:
        //                     sys_save_state = false
        //             else if line.find("WIP:") == 0:  // found so begin start system read
        //                 wip_in_progress = true
        //             else if line.find("Romset:") == 0:  // found so begin start system read
        //                 wip_in_progress = false
        //                 romset_in_progress = true
        //             else:
        //                 if wip_in_progress and line.find("Romset:") != 0:
        // sys_wip += line[:-1] + "<BR>"
        //                     pass
        //                 if romset_in_progress and line.find("$end") != 0:
        // sys_romset += line[:-1] + "<BR>"
        //                     pass
        //                 if desc_next:
        //                     sys_desc = line
        //                     desc_next = false
        //                 if long_name_next:
        //                     try:
        //                         sys_longname, sys_manufacturer, sys_year = line.split(",")
        //                     except:
        //                         sys_longname, msys_manufacturer, sys_year = line.rsplit(",", 2)
        //                     long_name_next = false
        //                     desc_next = true
        //                 if line.find("$end") == 0:  // end of system info so store system into db
        //                     romset_in_progress = false
        //                     if sys_desc[:-1] == "...":
        //                         sys_desc = None
        //                     else:
        //                         sys_desc = sys_desc[:-1]
        //                     sys_emulation = sys_emulation[:-1]
        //                     sys_color = sys_color[:-1]
        //                     sys_sound = sys_sound[:-1]
        //                     sys_graphics = sys_graphics[:-1]
        // upsert the system
        //                     db_connection.db_meta_game_system_upsert(sys_short_name[:-1],
        //                                                              sys_longname,
        //                                                              sys_desc, sys_year[:-1],
        //                                                              sys_manufacturer,
        //                                                              sys_emulation,
        //                                                              sys_color, sys_sound,
        //                                                              sys_graphics, sys_save_state)
        //                     sys_wip = None
        //                     sys_romset = None
    }
    Ok(())
}
