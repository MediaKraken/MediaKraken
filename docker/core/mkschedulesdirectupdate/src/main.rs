use chrono::prelude::*;
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use stdext::function_name;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    let _db_check =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkschedulesdirectupdate")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mkschedulesdirectupdate",
        &rabbit_channel,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "msg body": json_message }),
                    )
                    .await
                    .unwrap();
                }
                //
                // def mk_schedules_direct_program_info_fetch(meta_program_fetch):
                //     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                //                                                          message_text={'array': meta_program_fetch})
                //     meta_program_json = sd.com_schedules_direct_program_info(
                //         json.dumps(meta_program_fetch))
                //     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                //                                                          message_text={'result': meta_program_json})
                //     #   meta_program_json = sd.com_Schedules_Direct_Program_Desc(
                //     # json.dumps([{'programID': program_json['programID']}]))
                //     for program_data in meta_program_json:
                //         db_connection.db_tv_program_insert(
                //             program_json['programID'], json.dumps(program_data))
                //
                //
                // sd = common_schedules_direct.CommonSchedulesDirect()
                // sd.com_schedules_direct_login(option_config_json['SD']['User'],
                //                               option_config_json['SD']['Password'])
                // status_data = sd.com_schedules_direct_status()
                // if status_data['systemStatus'][0]['status'] == "Online":
                //     pass
                // else:
                //     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='critical',
                //                                                          message_text={'stuff': 'SD is unavailable'})
                //     sys.exit(0)
                // # version check
                // # version_json = sd.com_Schedules_Direct_Client_Version()
                // // TODO
                // # if version_json != "MediaKraken_0.1.0":
                // #    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='critical', message_text= {'stuff':"Outdated Client Version! Upgrade MediaKraken_")
                // #    sys.exit(0)
                //
                // # get headends
                // # headends_json = sd.com_Schedules_Direct_Headends('USA', '58701')
                //
                // # add to lineup
                // # common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':sd.com_Schedules_Direct_Lineup_Add('USA-ND33420-DEFAULT'))
                //
                // # remove from lineup
                // # common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':sd.com_Schedules_Direct_Lineup_Delete('USA-DISH687-DEFAULT'))
                //
                //
                // # list lineups and channels for them
                // # for line_name in sd.com_Schedules_Direct_Lineup_List()['lineups']:
                // #    # channel map
                // #    channel_map = sd.com_Schedules_Direct_Lineup_Channel_Map(line_name['lineup'])
                // #    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':"Map: %s", channel_map['map'])
                // #    for channel_id in channel_map['map']:
                // #        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':"mapchannel: %s", channel_id)
                // #        db_connection.db_tv_station_insert(channel_id['stationID'], channel_id['channel'])
                // #    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':"Stations: %s", channel_map['stations'])
                // #    for channel_meta in channel_map['stations']:
                // #        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'stuff':"stationschannel: %s", channel_meta)
                // #        db_connection.db_tv_station_update(channel_meta['name'], channel_meta['stationID'],\
                // # json.dumps(channel_meta))
                //
                //
                // // TODO downloading a generic description of a program
                // # - good for what the show is......not an episode itself
                //
                // station_fetch = []
                // common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
                //     'list': db_connection.db_tv_stations_read_stationid_list()})
                // # grab all stations in DB
                // for station_id in db_connection.db_tv_stations_read_stationid_list():
                //     # fetch all schedules for station
                //     station_fetch.append(station_id['mm_tv_station_id'])
                //
                // # set here so it exists at the "end" of processing
                // meta_program_fetch = []
                // # grab station info from SD
                // if len(station_fetch) > 5000:
                //     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='critical', message_text=
                //                                             {'stuff': 'Too many channels!!!!  Exiting...'})
                // else if len(station_fetch) > 0:
                //     schedule_json = sd.com_schedules_direct_schedules_by_stationid(
                //         json.dumps(station_fetch))
                //     # for each station in schedules results
                //     for station_json in schedule_json:
                //         # [{u'stationID': u'10093', u'metadata': {u'startDate': u'2016-06-15',
                //         # u'modified': u'2016-06-14T23:07:05Z', u'md5': u'2aEwFuhZCqJSHKabBbR/Sg'},
                //         meta_program_fetch = []
                //         # for each program in station schedule result
                //         for program_json in station_json['programs']:
                //             # {u'ratings': [{u'body': u'USA Parental Rating', u'code': u'TV14'}],
                //             # u'audioProperties': [u'DD 5.1', u'stereo'], u'duration': 9000,
                //             # u'programID': u'MV000135600000', u'airDateTime': u'2016-06-15T00:30:00Z',
                //             #  u'md5': u'18/KxBZUiJQu5sCix7WWwQ'},
                //             db_connection.db_tv_schedule_insert(station_json['stationID'],
                //                                                 program_json['airDateTime'],
                //                                                 json.dumps(program_json))
                //             common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text=
                //             {'programid': program_json['programID']})
                //             # if program_json['programID'][0:2] != "MV":
                //             meta_program_fetch.append(program_json['programID'])
                //             if len(meta_program_fetch) >= 500:
                //                 mk_schedules_direct_program_info_fetch(meta_program_fetch)
                //                 meta_program_fetch = []
                //
                // // TODO check to see if meta array has unstored data
                // if len(meta_program_fetch) > 0:
                //     mk_schedules_direct_program_info_fetch(meta_program_fetch)
                //
                // // TODO, go grab images for blank logos
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
