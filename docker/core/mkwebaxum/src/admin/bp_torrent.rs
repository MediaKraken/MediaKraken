use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use bytesize::ByteSize;
use core::fmt::Write;
use mk_lib_common;
use mk_lib_database;
use mk_lib_network;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_torrent.html")]
struct AdminTorrentTemplate<'a> {
    template_data_host: &'a String,
}

pub async fn admin_torrent(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    // let transmission_client =
    //     mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login()
    //         .await
    //         .unwrap();

    // let transmission_torrents =
    //     mk_lib_network::mk_lib_network_transmission::mk_network_transmission_list_torrents(
    //         transmission_client,
    //     )
    //     .await
    //     .unwrap();
    let docker_results = mk_lib_common::mk_lib_common_docker::mk_common_docker_info()
        .await
        .unwrap();
    let template = AdminTorrentTemplate {
        template_data_host: &docker_results.name.unwrap(),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*

@blueprint_admin_torrent.route('/admin_torrent_delete', methods=["POST"])
pub async fn url_bp_admin_torrent_delete(request):
    """
    Delete torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_torrent.route('/admin_torrent_edit', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_torrent_edit(request):
    """
    Edit a torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})

 */
