#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use serde_json::Value;
use sqlx::Row;
use std::error::Error;
use stdext::function_name;
use uuid::Uuid;

#[path = "database"]
pub mod database {
    pub mod mk_lib_database;
    pub mod mk_lib_database_option_status;
    pub mod mk_lib_database_version;
    pub mod mk_lib_database_version_schema;
}

mod mk_lib_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // connect to db and do a version check
    let sqlx_pool = database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false).await;

    let option_config_json: Value =
        database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    /*

    GAMESDB_INST = metadata_provider_thegamesdb.CommonMetadataGamesDB()

    # grab and insert all platforms
    for platform in list(GAMESDB_INST.com_meta_gamesdb_platform_list()['Data']['Platforms'].items())[0]:
        if platform != 'Platform':
            for game_systems in platform:
                print(game_systems, flush=True)
                if db_connection.db_meta_games_system_guid_by_short_name(game_systems['name']) == None:
                    # fetch platform info
                    platform_json = GAMESDB_INST.com_meta_gamesdb_platform_by_id(game_systems['id'])
                    # store record
                    try:
                        system_alias = game_systems['alias']
                    except KeyError:
                        system_alias = None
                    db_connection.db_meta_games_system_upsert(game_systems['id'],
                                                              game_systems['name'],
                                                              system_alias,
                                                              json.dumps(platform_json))
    */
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"}))
            .await
            .unwrap();
    }
    Ok(())
}
