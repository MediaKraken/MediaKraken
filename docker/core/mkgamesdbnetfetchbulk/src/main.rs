use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await;

    let _option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkgamesdbnetfetchbulk")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mkgamesdbnetfetchbulk",
        &rabbit_channel,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                // #[cfg(debug_assertions)]
                // {
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({ "msg body": json_message }),
                //     )
                //     .await
                //     .unwrap();
                // }
                /*
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
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
