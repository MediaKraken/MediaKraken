#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::fs::File;
use async_std::path::PathBuf;
use chrono::prelude::*;
use serde_json::{json, Value};
use std::error::Error;
use std::path::Path;
use tokio::time::{Duration, sleep};
use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use std::io::{self, prelude::*, BufReader};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

// https://www.progettosnaps.net/download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats_236.7z

#[path = "mk_lib_compression.rs"]
mod mk_lib_compression;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;
#[path = "mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;

// technically arcade games are "systems"....
// they just don"t have @isdevice = "yes" like mess hardware does

// However, mame games are still being put as "games" and not systems
// to ease search and other filters by game/system

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkmetadatamame";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // open the database
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;
    let option_config_json: Value = mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool).await.unwrap();

    // create mame game list
    let file_name = format!("/mediakraken/emulation/mame0{}lx.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists()
    {
        mk_lib_network::mk_download_file_from_url(
            format!("https://github.com/mamedev/mame/releases/download/mame0{}/mame0{}lx.zip",
                    option_config_json["MAME"]["Version"],
                    option_config_json["MAME"]["Version"]), &file_name).await;
        let unzip_file_name = format!("/mediakraken/emulation/mame0{}.xml", option_config_json["MAME"]["Version"]);
        if !Path::new(&unzip_file_name).exists()
        {
            mk_lib_compression::mk_decompress_zip(&file_name,
                                                  true,
                                                  false,
                                                  "/mediakraken/emulation/").unwrap();
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
                    // TODO this really needs to be an upsert
                    mk_lib_database_metadata_game::mk_lib_database_metadata_game_insert(
                        &sqlx_pool, uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
                        json_data["machine"]["@name"].to_string(),
                        json_data["machine"]["description"].to_string(),
                        json_data).await;
                } else {
                    xml_data.push_str(xml_line);
                }
            }
        }
    }

    // load games from hash files
    let file_name = format!("/mediakraken/emulation/mame0{}.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!("https://github.com/mamedev/mame/archive/mame0{}.zip",
                    option_config_json["MAME"]["Version"]),
            &file_name);
        //     zip_handle = zipfile.ZipFile(file_name, "r")  # issues if u do RB
        //     zip_handle.extractall("/mediakraken/emulation/")
        //     zip_handle.close()
        //     for zippedfile in os.listdir(format!("/mediakraken/emulation/mame-mame0{}/hash",
        //                                  option_config_json["MAME"]["Version"])):
        // find system id from mess
        //         file_name, ext = os.path.splitext(zippedfile)
        //         if ext == ".xml" or ext == ".hsi":
        //             file_handle = open(os.path.join(format!("/mediakraken/emulation/mame-mame0{}/hash",
        //                                              option_config_json["MAME"]["Version"]), zippedfile),
        //                                "r",
        //                                encoding="utf-8")
        //             json_data = xmltodict.parse(file_handle.read())
        //             file_handle.close()
        //             game_short_name_guid \
        //                 = db_connection.db_meta_games_system_guid_by_short_name(file_name)
        //             if game_short_name_guid == None:
        //                 game_short_name_guid = db_connection.db_meta_games_system_insert(
        //                     None, file_name, None)
        //             if ext == ".xml":
        // could be no games in list
        //                 if "software" in json_data["softwarelist"]:
        // TODO this fails if only one game
        //                     if "@name" in json_data["softwarelist"]["software"]:
        // TODO check to see if exists....upsert instead
        //                         db_connection.db_meta_game_insert(game_short_name_guid,
        //                                                           json_data["softwarelist"]["software"][
        //                                                               "@name"],
        //                                                           json_data["softwarelist"]["software"][
        //                                                               "@name"],
        //                                                           json_data["softwarelist"]["software"])
        //                     else:
        //                         for json_game in json_data["softwarelist"]["software"]:
        // json_game = json.loads(json_game)
        // TODO check to see if exists....upsert instead
        // build args and insert the record
        //                             db_connection.db_meta_game_insert(game_short_name_guid,
        //                                                               json_game["@name"],
        //                                                               json_game["@name"], json_game)
        //             else if ext == ".hsi":
        // could be no games in list
        //                 if "hash" in json_data["hashfile"]:
        //                     if "@name" in json_data["hashfile"]["hash"]:
        // TODO check to see if exists....upsert instead
        //                         db_connection.db_meta_game_insert(game_short_name_guid,
        //                                                           json_data["hashfile"]["hash"][
        //                                                               "@name"],
        //                                                           json_data["hashfile"]["hash"][
        //                                                               "@name"],
        //                                                           json_data["hashfile"]["hash"])
        //                     else:
        //                         for json_game in json_data["hashfile"]["hash"]:
        // TODO check to see if exists....upsert instead
        // build args and insert the record
        //                             db_connection.db_meta_game_insert(game_short_name_guid,
        //                                                               json_game["@name"],
        //                                                               json_game["@name"], json_game)
    }

    // update mame game descriptions from history dat
    let file_name = format!("/mediakraken/emulation/history{}.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!("https://www.arcade-history.com/dats/historydat{}.zip",
                    option_config_json["MAME"]["Version"]),
            &file_name);
        let mut game_titles = Vec::new();;
        let mut game_desc = "".to_string();
        let mut add_to_desc = false;
        let mut new_title = "".to_string();
        let mut system_name = "".to_string();
        // do this all the time, since could be a new one
        //     with zipfile.ZipFile(file_name, "r") as zf:
        //         zf.extract("history.dat", "/mediakraken/emulation/")
        //     history_file = open("/mediakraken/emulation/history.dat", "r",
        //                         encoding="utf-8")
        //     while 1:
        //         line = history_file.readline()
        //         // print("line: %s" % line, flush=True)
        //         if not line:
        //             break
        //         if line[0] == "$" and line[-1:] == ",":  // this could be a new system/game item
        // MAME "system"....generally a PCB game
        //             if line.find("$info=") == 0:  # goes by position if found
        //                 system_name = None
        //                 game_titles = line.split("=", 1)[1].split(",")
        // end of info block for game
        //             else if line.find("$end") == 0:  // goes by position if found
        //                 add_to_desc = false
        //                 for game in game_titles:
        //                     game_data = db_connection.db_meta_game_by_name_and_system(game, system_name)[0]
        //                     if game_data == None:
        //                         db_connection.db_meta_game_insert(
        //                             db_connection.db_meta_games_system_guid_by_short_name(
        //                                 system_name),
        //                             new_title, game, json.dumps({"overview": game_desc}))
        //                     else:
        //                         game_data["gi_game_info_json"]["overview"] = game_desc
        //                         db_connection.db_meta_game_update_by_guid(game_data["gi_id"],
        //                                                                   json.dumps(game_data[
        //                                                                                  "gi_game_info_json"]))
        //                 game_desc = ""
        // this line can be skipped and is basically the "start" of game info
        //             else if line.find("$bio") == 0:  // goes by position if found
        //                 line = history_file.readline()  // skip blank line
        //                 new_title = history_file.readline().strip()  // grab the "real" game name
        //                 add_to_desc = true
        //             else:
        // should be a system/game
        //                 system_name = line[1:].split("=", 1)[0]
        //                 game_titles = line.split("=", 1)[1].split(",")
        //         else:
        //             if add_to_desc:
        //                 game_desc += line
        //     history_file.close()
    }

    // read the category file and create dict/list for it
    let file_name = format!("/mediakraken/emulation/category{}.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
                format!("https://www.progettosnaps.net/download?tipo=category&file=/renameset/packs/pS_category_{}.zip",
                        option_config_json["MAME"]["Version"]),
            &file_name);

        //     with zipfile.ZipFile(file_name, "r") as zf:
        //         zf.extract("folders/category.ini", "/mediakraken/emulation/")
        //     history_file = open("/mediakraken/emulation/category.ini", "r",
        //                         encoding="utf-8")
        //     cat_file = open("category.ini", "r", encoding="utf-8")
        //     cat_dictionary = {}
        //     category = ""
        //     while 1:
        //         line = cat_file.readline()
        //         if not line:
        //             break
        //         if line.find("[") == 0:
        //             category = line.replace("[", "").replace("]", "").replace(" ", "").rstrip("\n").rstrip(
        //                 "\r")  # wipe out space to make the category table
        //         else if len(line) > 1:
        //             result_value = db_connection.db_meta_game_category_by_name(category)
        //             if result_value == None:
        //                 result_value = db_connection.db_meta_game_category_add(category)
        //             cat_dictionary[line.strip()] = result_value
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

    // update mess system description
    let file_name = format!("/mediakraken/emulation/messinfo{}.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists() {
        mk_lib_network::mk_download_file_from_url(
            format!(
                "https://www.progettosnaps.net/download?tipo=messinfo&file=pS_messinfo_{}.zip",
                option_config_json["MAME"]["Version"]),
            &file_name);
        //     with zipfile.ZipFile(file_name, "r") as zf:
        //         zf.extract("messinfo.dat", "/mediakraken/emulation/")
        //     infile = open("/mediakraken/emulation/messinfo.dat", "r",
        //                   encoding="utf-8")
        let mut start_system_read = false;
        let mut skip_next_line = false;
        let mut long_name_next = false;
        let mut desc_next = false;
        let mut wip_in_progress = false;
        let mut romset_in_progress = false;
        // store args to sql
        let mut sys_short_name = "".to_string();
        let mut sys_longname = "".to_string();
        let mut sys_manufacturer = "".to_string();
        let mut sys_year: i8 = 0;
        let mut sys_desc = "".to_string();
        let mut sys_emulation = "".to_string();
        let mut sys_color = "".to_string();
        let mut sys_sound = "".to_string();
        let mut sys_graphics = "".to_string();
        let mut sys_save_state = "".to_string();
        let mut sys_wip = "".to_string();
        let mut sys_romset = "".to_string();

        let mut sql_string = "".to_string();
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