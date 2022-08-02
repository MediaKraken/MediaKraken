#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use sqlx::Row;

#[path = "../mk_lib_database_media.rs"]
mod mk_lib_database_media;

#[path = "../mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;

#[path = "../mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[path = "../mk_lib_network.rs"]
mod mk_lib_network;

#[derive(Serialize)]
struct TemplateHomeStreamListContext {
    template_stream_user: String,
    template_stream_media: String,
    template_stream_device: String,
    template_stream_time: String,
}

#[derive(Serialize)]
struct TemplateHomeScanListContext {
    template_scan_path: String,
    template_scan_status: String,
    template_scan_percentage: String,
}

#[derive(Serialize)]
struct TemplateHomeContext {
    template_data_server_info_server_name: String,
    template_data_server_uptime: String,
    template_data_server_host_ip: String,
    template_data_server_info_server_ip_external: String,
    template_data_server_info_server_version: String,
    template_data_count_media_files: i64,
    template_data_count_matched_media: i64,
    template_data_count_meta_fetch: i64,
    template_data_count_streamed_media: i32,
    template_server_streams: Vec<TemplateHomeStreamListContext>,
    template_server_users: Vec<mk_lib_database_user::DBUserList>,
    template_data_scan_info: Vec<TemplateHomeScanListContext>,
}

#[get("/home")]
pub async fn admin_home(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    let user_list = mk_lib_database_user::mk_lib_database_user_read(&sqlx_pool, 0, 9999)
        .await
        .unwrap();
    let option_status_row =
        mk_lib_database_option_status::mk_lib_database_option_status_read(&sqlx_pool)
            .await
            .unwrap();
    let option_json: serde_json::Value = option_status_row.get("mm_options_json");
    let status_json: serde_json::Value = option_status_row.get("mm_status_json");
    let boot_seconds: libc::timeval = sys_info::boottime().unwrap();
    let boot_duration = chrono::Duration::seconds(i64::from(boot_seconds.tv_sec));
    let external_ip = mk_lib_network::mk_data_from_url("https://myexternalip.com/raw".to_string())
        .await
        .unwrap();
    let mut server_streams = Vec::new();
    let mut server_scans = Vec::new();
    Template::render(
        "bss_admin/bss_admin_home",
        &TemplateHomeContext {
            template_data_server_info_server_name: option_json["MediaKrakenServer"]["Server Name"]
                .to_string(),
            // following boottime only compiles #[cfg(not(windows))] in this case is fine
            template_data_server_uptime: format!(
                "{:02}:{:02}:{:02}",
                boot_duration.num_hours(),
                boot_duration.num_minutes() % 60,
                boot_duration.num_seconds() % 60
            ),
            template_data_server_host_ip: "255.255.255.255".to_string(),
            template_data_server_info_server_ip_external: external_ip,
            template_data_server_info_server_version: "Fake Version".to_string(),
            template_data_count_media_files:
                mk_lib_database_media::mk_lib_database_media_known_count(&sqlx_pool)
                    .await
                    .unwrap(),
            template_data_count_matched_media:
                mk_lib_database_media::mk_lib_database_media_matched_count(&sqlx_pool)
                    .await
                    .unwrap(),
            template_data_count_meta_fetch:
                mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_count(
                    &sqlx_pool,
                )
                .await
                .unwrap(),
            template_data_count_streamed_media: 0,
            template_server_streams: server_streams,
            template_server_users: user_list,
            template_data_scan_info: server_scans,
        },
    )
}

/*
pub async fn url_bp_admin(request):
    data_server_info_server_name = 'Spoots Media'
    nic_data = []
    for key, value in common_network.mk_network_ip_addr().items():
        nic_data.append(key + ' ' + value[0][1])
    data_alerts_dismissable = []
    data_alerts = []
    # read in the notifications
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_notification_read(
            db_connection=db_connection):
        if row_data['mm_notification_dismissable']:  # check for dismissable
            data_alerts_dismissable.append((row_data['mm_notification_guid'],
                                            row_data['mm_notification_text'],
                                            row_data['mm_notification_time']))
        else:
            data_alerts.append((row_data['mm_notification_guid'],
                                row_data['mm_notification_text'],
                                row_data['mm_notification_time']))
    data_transmission_active = False
    row_data = await request.app.db_functions.db_opt_json_read(db_connection=db_connection)
    if row_data['Docker Instances']['transmission'] is True:
        data_transmission_active = True
    # set the scan info
    data_scan_info = []
    scanning_json = await request.app.db_functions.db_status_json_read(db_connection=db_connection)
    if 'Status' in scanning_json:
        data_scan_info.append(('System', scanning_json['Status'], scanning_json['Pct']))
    for row_data in await request.app.db_functions.db_library_path_status(
            db_connection=db_connection):
        data_scan_info.append((row_data['mm_media_dir_path'],
                               row_data['mm_media_dir_status']['Status'],
                               row_data['mm_media_dir_status']['Pct']))
    if os.environ['SWARMIP'] != 'None':
        mediakraken_ip = os.environ['SWARMIP']
    else:
        mediakraken_ip = os.environ['HOST_IP']
    user_count = common_internationalization.com_inter_number_format(
        await request.app.db_functions.db_user_count(db_connection=db_connection))
    media_file_count = common_internationalization.com_inter_number_format(
        await request.app.db_functions.db_media_known_count(db_connection=db_connection))
    media_matched_count = common_internationalization.com_inter_number_format(
        await request.app.db_functions.db_media_matched_count(db_connection=db_connection))
    media_library_count = common_internationalization.com_inter_number_format(
        await request.app.db_functions.db_table_count(table_name='mm_media_dir',
                                                      db_connection=db_connection))
    metadata_to_fetch = common_internationalization.com_inter_number_format(
        await request.app.db_functions.db_table_count(table_name='mm_download_que',
                                                      db_connection=db_connection))
    await request.app.db_pool.release(db_connection)
    return {
        'data_user_count': user_count,
        'data_server_info_server_name': data_server_info_server_name,
        'data_host_ip': mediakraken_ip,
        'data_server_info_server_ip': nic_data,
        'data_server_info_server_ip_external': outside_ip,
        'data_server_info_server_version': common_version.APP_VERSION,
        'data_server_uptime': common_system.com_system_uptime(),
        'data_active_streams': common_internationalization.com_inter_number_format(0),
        'data_alerts_dismissable': data_alerts_dismissable,
        'data_alerts': data_alerts,
        'data_count_media_files': media_file_count,
        'data_count_matched_media': media_matched_count,
        'data_count_streamed_media': common_internationalization.com_inter_number_format(0),
        'data_library': media_library_count,
        'data_transmission_active': data_transmission_active,
        'data_scan_info': data_scan_info,
        'data_count_meta_fetch': metadata_to_fetch,
    }
 */
