#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::Row;
use uuid::Uuid;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkgamesdbnetfetchbulk";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    let option_config_json: Value = mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool).await.unwrap();
}

/*

GAMESDB_INST = metadata_provider_thegamesdb.CommonMetadataGamesDB()

# grab and insert all platforms
for platform in \
        list(GAMESDB_INST.com_meta_gamesdb_platform_list()['Data']['Platforms'].items())[0]:
    if platform != 'Platform':
        for game_systems in platform:
            print(game_systems, flush=True)
            if db_connection.db_meta_games_system_guid_by_short_name(game_systems['name']) is None:
                # fetch platform info
                platform_json = GAMESDB_INST.com_meta_gamesdb_platform_by_id(game_systems['id'])
                # store record
                try:
                    system_alias = game_systems['alias']
                except KeyError:
                    system_alias = None
                db_connection.db_meta_games_system_insert(game_systems['id'],
                                                          game_systems['name'],
                                                          system_alias,
                                                          json.dumps(platform_json))
                db_connection.db_commit()
 */