use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, Path},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_network;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_torrent.html")]
struct AdminTorrentTemplate<'a> {
    template_data_json: &'a str,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct TorrentAddInput {
    file_url: String,
}

async fn user_can_admin_torrent(
    method: &Method,
    auth: &AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> bool {
    let current_user = auth.current_user.clone().unwrap_or_default();
    Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET, Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, method, None)
    .await
}

pub async fn admin_torrent(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    if !user_can_admin_torrent(&method, &auth).await {
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
        let transmission_torrents_json = match serde_json::to_string(&transmission_torrents) {
            Ok(value) => value,
            Err(_) => "[]".to_string(),
        };
        let template = AdminTorrentTemplate {
            template_data_json: &transmission_torrents_json,
            page_title: Some("MediaKraken Admin Torrent".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn admin_torrent_add(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<TorrentAddInput>,
) -> impl IntoResponse {
    if !user_can_admin_torrent(&method, &auth).await {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    let file_url = input_data.file_url.trim().to_string();
    if file_url.is_empty() {
        return (StatusCode::BAD_REQUEST, "file_url is required.").into_response();
    }
    let transmission_client =
        match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login().await {
            Ok(client) => client,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to connect to transmission.",
                )
                    .into_response();
            }
        };
    match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_add_torrent(
        transmission_client,
        file_url,
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::BAD_GATEWAY, "Transmission rejected add.").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unable to add torrent.").into_response(),
    }
}

pub async fn admin_torrent_start(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(torrent_id): Path<i64>,
) -> impl IntoResponse {
    if !user_can_admin_torrent(&method, &auth).await {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    let transmission_client =
        match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login().await {
            Ok(client) => client,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to connect to transmission.",
                )
                    .into_response();
            }
        };
    match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_start_torrent(
        transmission_client,
        torrent_id,
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::BAD_GATEWAY, "Transmission rejected start.").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to start torrent.",
        )
            .into_response(),
    }
}

pub async fn admin_torrent_stop(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(torrent_id): Path<i64>,
) -> impl IntoResponse {
    if !user_can_admin_torrent(&method, &auth).await {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    let transmission_client =
        match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login().await {
            Ok(client) => client,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to connect to transmission.",
                )
                    .into_response();
            }
        };
    match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_stop_torrent(
        transmission_client,
        torrent_id,
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::BAD_GATEWAY, "Transmission rejected stop.").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unable to stop torrent.").into_response(),
    }
}

pub async fn admin_torrent_delete(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(torrent_id): Path<i64>,
) -> impl IntoResponse {
    if !user_can_admin_torrent(&method, &auth).await {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    let transmission_client =
        match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login().await {
            Ok(client) => client,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to connect to transmission.",
                )
                    .into_response();
            }
        };
    match mk_lib_network::mk_lib_network_transmission::mk_network_transmission_remove_torrent(
        transmission_client,
        torrent_id,
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::BAD_GATEWAY, "Transmission rejected delete.").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to delete torrent.",
        )
            .into_response(),
    }
}
