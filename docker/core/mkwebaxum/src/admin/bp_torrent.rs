use crate::axum_custom_filters::filters;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use bytesize::ByteSize;
use core::fmt::Write;
use mk_lib_common;
use crate::mk_lib_database;
use mk_lib_network;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_torrent.html")]
struct AdminTorrentTemplate<'a> {
    template_data: &'a Vec<mk_lib_network::mk_lib_network_transmission::TorrentList>,
}

pub async fn admin_torrent(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let transmission_client =
            mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login()
                .await
                .unwrap();
        let transmission_torrents =
            mk_lib_network::mk_lib_network_transmission::mk_network_transmission_list_torrents(
                transmission_client,
            )
            .await
            .unwrap();
        let template = AdminTorrentTemplate {
            template_data: &transmission_torrents,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
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
