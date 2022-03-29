use async_std::path::PathBuf;
use chrono::prelude::*;
use serde_json::{json, Value};
use std::error::Error;
use std::path::Path;
use tokio::time::{Duration, sleep};
use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use std::io::{self, prelude::*, BufReader};

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

    let mut update_game: i32 = 0;
    let mut insert_game: i32 = 0;

    // create mame game list
    let file_name = format!("/mediakraken/emulation/mame0{}lx.zip",
                            option_config_json["MAME"]["Version"]);
    // only do the parse/import if not processed before
    if !Path::new(&file_name).exists()
    {
        println!("download {}", file_name);
        mk_lib_network::mk_download_file_from_url(
            format!("https://github.com/mamedev/mame/releases/download/mame0{}/mame0{}lx.zip",
                    option_config_json["MAME"]["Version"],
                    option_config_json["MAME"]["Version"]), &file_name).await;
        println!("here2");
    }
    println!("b4 xml");
    let unzip_file_name = format!("/mediakraken/emulation/mame0{}.xml", option_config_json["MAME"]["Version"]);
    if !Path::new(&unzip_file_name).exists()
    {
        println!("b4 unzip");
        mk_lib_compression::mk_decompress_zip(&file_name,
                                              true,
                                              false,
                                              "/mediakraken/emulation/").unwrap();
    }
    println!("b4 file load");
    let file = File::open(&unzip_file_name)?;
    let reader = BufReader::new(file);
    let mut xml_data: String = "".to_string();
    let conf = Config::new_with_custom_values(true, "", "text", NullValue::Ignore)
        .add_json_type_override("/machine/@name", JsonArray::Infer(JsonType::AlwaysString))
        .add_json_type_override("/year", JsonArray::Infer(JsonType::AlwaysString))
        .add_json_type_override("/manufacturer", JsonArray::Infer(JsonType::AlwaysString));
    println!("b4 read lines");
    for line in reader.lines() {
        let xml_line = &line.unwrap().trim().to_string();
        if xml_line.starts_with("<machine") == true {
            println!("here");
            xml_data = xml_line.to_string();
        }
        else if xml_line.starts_with("</machine") == true {
            xml_data += xml_line;
            println!("xml {}", xml_data);
            let json_data = xml_string_to_json(xml_data.to_string(), &conf);
            println!("json {:?}", json_data.unwrap());
            //println!("json {:?}", json.unwrap()["machine"]["@name"]);
            // TODO change this to upsert
            //     db_connection.db_meta_game_insert(None, json_data["machine"]["@name"],
            //                                       json_data["machine"]["description"],
            //                                       json_data)
        }
        else {
            xml_data += xml_line;
        }
    }

    // write totals
    //     if update_game > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(update_game)
    //             + " games(s) metadata updated from MAME %s XML" % option_config_json["MAME"]["Version"],
    //             true)
    //     if insert_game > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(insert_game)
    //             + " games(s) metadata added from MAME %s XML" % option_config_json["MAME"]["Version"],
    //             true)



    // load games from hash files
    // file_name = ("/mediakraken/emulation/mame0%s.zip" %
    //              option_config_json["MAME"]["Version"])
    // only do the parse/import if not processed before
    // if not os.path.exists(file_name):
    //     mk_lib_network::mk_download_file_from_url(
    //         ("https://github.com/mamedev/mame/archive/mame0%s.zip"
    //          % option_config_json["MAME"]["Version"]),
    //         file_name)
    let total_software: i32 = 0;
    let total_software_update: i32 = 0;
    //     zip_handle = zipfile.ZipFile(file_name, "r")  # issues if u do RB
    //     zip_handle.extractall("/mediakraken/emulation/")
    //     zip_handle.close()
    //     for zippedfile in os.listdir("/mediakraken/emulation/mame-mame0%s/hash"
    //                                  % option_config_json["MAME"]["Version"]):
    //         # find system id from mess
    //         file_name, ext = os.path.splitext(zippedfile)
    //         print("fil,etx %s %s" % (file_name, ext), flush=True)
    //         if ext == ".xml" or ext == ".hsi":
    //             file_handle = open(os.path.join("/mediakraken/emulation/mame-mame0%s/hash"
    //                                             % option_config_json["MAME"]["Version"], zippedfile),
    //                                "r",
    //                                encoding="utf-8")
    //             json_data = xmltodict.parse(file_handle.read())
    //             file_handle.close()
    //             game_short_name_guid \
    //                 = db_connection.db_meta_games_system_guid_by_short_name(file_name)
    //             print("wh %s" % game_short_name_guid, flush=True)
    //             if game_short_name_guid is None:
    //                 game_short_name_guid = db_connection.db_meta_games_system_insert(
    //                     None, file_name, None)
    //             print("json: %s" % json_data, flush=True)
    //             if ext == ".xml":
    //                 # could be no games in list
    //                 if "software" in json_data["softwarelist"]:
    //                     print(json_data["softwarelist"]["software"], flush=True)
    //                     # TODO this fails if only one game
    //                     print(len(json_data["softwarelist"]["software"]), flush=True)
    //                     if "@name" in json_data["softwarelist"]["software"]:
    //                         # TODO check to see if exists....upsert instead
    //                         db_connection.db_meta_game_insert(game_short_name_guid,
    //                                                           json_data["softwarelist"]["software"][
    //                                                               "@name"],
    //                                                           json_data["softwarelist"]["software"][
    //                                                               "@name"],
    //                                                           json_data["softwarelist"]["software"])
    //                     else:
    //                         for json_game in json_data["softwarelist"]["software"]:
    //                             print(("xml: %s", json_game), flush=True)
    //                             # json_game = json.loads(json_game)
    //                             # TODO check to see if exists....upsert instead
    //                             # build args and insert the record
    //                             db_connection.db_meta_game_insert(game_short_name_guid,
    //                                                               json_game["@name"],
    //                                                               json_game["@name"], json_game)
    //                     total_software += 1
    //             elif ext == ".hsi":
    //                 # could be no games in list
    //                 if "hash" in json_data["hashfile"]:
    //                     if "@name" in json_data["hashfile"]["hash"]:
    //                         # TODO check to see if exists....upsert instead
    //                         db_connection.db_meta_game_insert(game_short_name_guid,
    //                                                           json_data["hashfile"]["hash"][
    //                                                               "@name"],
    //                                                           json_data["hashfile"]["hash"][
    //                                                               "@name"],
    //                                                           json_data["hashfile"]["hash"])
    //                     else:
    //                         for json_game in json_data["hashfile"]["hash"]:
    //                             print("hsi: %s" % json_game, flush=True)
    //                             # TODO check to see if exists....upsert instead
    //                             # build args and insert the record
    //                             db_connection.db_meta_game_insert(game_short_name_guid,
    //                                                               json_game["@name"],
    //                                                               json_game["@name"], json_game)
    //                     total_software += 1
    //     if total_software > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(total_software)
    //             + " games(s) metadata added from MAME %s hash" % option_config_json["MAME"]["Version"],
    //             True)
    //     if total_software_update > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(
    //                 total_software_update)
    //             + " games(s) metadata updated from MAME %s hash" % option_config_json["MAME"][
    //                 "Version"], True)
    //     # commit all changes to db
    //     db_connection.db_commit()
    //
    // # update mame game descriptions from history dat
    // file_name = ("/mediakraken/emulation/history%s.zip" %
    //              option_config_json["MAME"]["Version"])
    // # only do the parse/import if not processed before
    // if not os.path.exists(file_name):
    //     mk_lib_network::mk_download_file_from_url(
    //         ("https://www.arcade-history.com/dats/historydat%s.zip" %
    //          option_config_json["MAME"]["Version"]),
    //         file_name)
    //     game_titles = []
    let mut game_desc = "";
    let mut add_to_desc = false;
    let mut new_title = "";
    let mut total_software: i32 = 0;
    let mut total_software_update: i32 = 0;
    let mut system_name = "";
    //     # do this all the time, since could be a new one
    //     with zipfile.ZipFile(file_name, "r") as zf:
    //         zf.extract("history.dat", "/mediakraken/emulation/")
    //     history_file = open("/mediakraken/emulation/history.dat", "r",
    //                         encoding="utf-8")
    //     while 1:
    //         line = history_file.readline()
    //         # print("line: %s" % line, flush=True)
    //         if not line:
    //             break
    //         if line[0] == "$" and line[-1:] == ",":  # this could be a new system/game item
    //             # MAME "system"....generally a PCB game
    //             if line.find("$info=") == 0:  # goes by position if found
    //                 system_name = None
    //                 game_titles = line.split("=", 1)[1].split(",")
    //             # end of info block for game
    //             elif line.find("$end") == 0:  # goes by position if found
    //                 add_to_desc = false
    //                 for game in game_titles:
    //                     print("game: %s" % game, flush=True)
    //                     game_data = db_connection.db_meta_game_by_name_and_system(game, system_name)[0]
    //                     print("data: %s" % game_data, flush=True)
    //                     if game_data is None:
    //                         db_connection.db_meta_game_insert(
    //                             db_connection.db_meta_games_system_guid_by_short_name(
    //                                 system_name),
    //                             new_title, game, json.dumps({"overview": game_desc}))
    //                         total_software += 1
    //                     else:
    //                         game_data["gi_game_info_json"]["overview"] = game_desc
    //                         print(game_data["gi_id"], flush=True)
    //                         db_connection.db_meta_game_update_by_guid(game_data["gi_id"],
    //                                                                   json.dumps(game_data[
    //                                                                                  "gi_game_info_json"]))
    //                         total_software_update += 1
    //                 game_desc = ""
    //             # this line can be skipped and is basically the "start" of game info
    //             elif line.find("$bio") == 0:  # goes by position if found
    //                 line = history_file.readline()  # skip blank line
    //                 new_title = history_file.readline().strip()  # grab the "real" game name
    //                 add_to_desc = true
    //             else:
    //                 # should be a system/game
    //                 system_name = line[1:].split("=", 1)[0]
    //                 game_titles = line.split("=", 1)[1].split(",")
    //         else:
    //             if add_to_desc:
    //                 game_desc += line
    //     history_file.close()
    //     if total_software > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(total_software)
    //             + " games(s) metadata added from MAME %s hash" % option_config_json["MAME"]["Version"],
    //             True)
    //     if total_software_update > 0:
    //         db_connection.db_notification_insert(
    //             common_internationalization.com_inter_number_format(
    //                 total_software_update)
    //             + " games(s) metadata updated from MAME %s hash" % option_config_json["MAME"][
    //                 "Version"], True)
    //     # commit all changes to db
    //     db_connection.db_commit()
    //
    // read the category file and create dict/list for it
    // file_name = ("/mediakraken/emulation/category%s.zip" %
    //              option_config_json["MAME"]["Version"])
    // only do the parse/import if not processed before
    // if not os.path.exists(file_name):
    //     mk_lib_network::mk_download_file_from_url(
    //         (
    //                 "https://www.progettosnaps.net/download?tipo=category&file=/renameset/packs/pS_category_%s.zip" %
    //                 option_config_json["MAME"]["Version"]),
    //         file_name)
    //
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
    //         elif len(line) > 1:
    //             result_value = db_connection.db_meta_game_category_by_name(category)
    //             if result_value is None:
    //                 result_value = db_connection.db_meta_game_category_add(category)
    //             cat_dictionary[line.strip()] = result_value
    //
    // grab all system null in db as those are mame
    //     for sql_row in db_connection.db_media_mame_game_list():
    //         db_connection.db_media_game_category_update(cat_dictionary[sql_row["gi_short_name"]],
    //                                                     sql_row["gi_id"])
    //
    // grab all the non parent roms that aren"t set
    //     for sql_row in db_connection.db_media_game_clone_list():
    //         for sql_cat_row in db_connection.db_media_game_category_by_name(sql_row["gi_cloneof"]):
    //             db_connection.db_media_game_category_update(sql_cat_row["gi_gc_category"],
    //                                                         sql_row["gi_id"])
    //
    // update mess system description
    // file_name = ("/mediakraken/emulation/messinfo%s.zip" %
    //              option_config_json["MAME"]["Version"])
    // only do the parse/import if not processed before
    // if not os.path.exists(file_name):
    //     mk_lib_network::mk_download_file_from_url(
    //         (
    //                 "https://www.progettosnaps.net/download?tipo=messinfo&file=pS_messinfo_%s.zip" %
    //                 option_config_json["MAME"]["Version"]),
    //         file_name)
    //
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
    let mut sys_short_name = "";
    let mut sys_longname = "";
    let mut sys_manufacturer = "";
    let mut sys_year: i8 = 0;
    let mut sys_desc = "";
    let mut sys_emulation = "";
    let mut sys_color = "";
    let mut sys_sound = "";
    let mut sys_graphics = "";
    let mut sys_save_state = "";
    let mut sys_wip = "";
    let mut sys_romset = "";
    //
    //     sql_string = ""
    //     while 1:
    //         line = infile.readline()
    //         if not line:
    //             break
    //         if skip_next_line:
    //             skip_next_line = false
    //         else:
    //             if line.find("DRIVERS INFO") != -1:  # stop at drivers
    //                 break
    //             line = line.replace("    ", "")
    //             if line[0] == "#" or len(line) < 4 \
    //                     or line.find("$mame") == 0:  # skip comments and blank lines
    //                 if line.find("$mame") == 0:
    //                     skip_next_line = true
    //                     long_name_next = true
    //             elif line.find("$info") == 0:  # found so begin start system read
    //                 start_system_read = true
    //                 # load the short name
    //                 sys_short_name = line.split("=")[1]
    //             elif line.find("Emulation:") == 0:  # found so begin start system read
    //                 sys_emulation = line.split(" ")[1]
    //             elif line.find("Color:") == 0:  # found so begin start system read
    //                 sys_color = line.split(" ")[1]
    //             elif line.find("Sound:") == 0:  # found so begin start system read
    //                 sys_sound = line.split(" ")[1]
    //             elif line.find("Graphics:") == 0:  # found so begin start system read
    //                 sys_graphics = line.split(" ")[1]
    //             elif line.find("Save State:") == 0:  # found so begin start system read
    //                 if line.rsplit(" ", 1)[1][:-1] == "Supported":
    //                     sys_save_state = true
    //                 else:
    //                     sys_save_state = false
    //             elif line.find("WIP:") == 0:  # found so begin start system read
    //                 wip_in_progress = true
    //             elif line.find("Romset:") == 0:  # found so begin start system read
    //                 wip_in_progress = false
    //                 romset_in_progress = true
    //             else:
    //                 if wip_in_progress and line.find("Romset:") != 0:
    //                     # sys_wip += line[:-1] + "<BR>"
    //                     pass
    //                 if romset_in_progress and line.find("$end") != 0:
    //                     // sys_romset += line[:-1] + "<BR>"
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
    //                     desc_next = True
    //                 if line.find("$end") == 0:  # end of system info so store system into db
    //                     romset_in_progress = false
    //                     if sys_desc[:-1] == "...":
    //                         sys_desc = None
    //                     else:
    //                         sys_desc = sys_desc[:-1]
    //                     sys_emulation = sys_emulation[:-1]
    //                     sys_color = sys_color[:-1]
    //                     sys_sound = sys_sound[:-1]
    //                     sys_graphics = sys_graphics[:-1]
    //                     // upsert the system
    //                     db_connection.db_meta_game_system_upsert(sys_short_name[:-1],
    //                                                              sys_longname,
    //                                                              sys_desc, sys_year[:-1],
    //                                                              sys_manufacturer,
    //                                                              sys_emulation,
    //                                                              sys_color, sys_sound,
    //                                                              sys_graphics, sys_save_state)
    //                     sys_wip = None
    //                     sys_romset = None
    //
    // commit all changes to db
    //     db_connection.db_commit()
    //
    // close the database
    // db_connection.db_close()
    Ok(())
}