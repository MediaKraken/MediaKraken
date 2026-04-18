use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::Flash;
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_server_links.html")]
struct TemplateAdminServerLinksContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_link_server::DBLinkList>,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct ServerLinkInput {
    host_or_ip: String,
    username: String,
    password: String,
}

pub async fn admin_server_links(
    State(state): State<AppState>,
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
        let server_links = mk_lib_database::mk_lib_database_link_server::mk_lib_database_link_read(
            &state.sqlx_pool_ro,
            0,
            100,
        )
        .await
        .unwrap_or_default();
        let template = TemplateAdminServerLinksContext {
            template_data: &server_links,
            page_title: Some("MediaKraken Admin Server Links".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn admin_server_links_post(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    mut flash: Flash,
    Form(input_data): Form<ServerLinkInput>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response()).into_response()
    } else {
        let host_or_ip = input_data.host_or_ip.trim();
        let username = input_data.username.trim();
        let password = input_data.password.trim();

        if host_or_ip.is_empty() || username.is_empty() || password.is_empty() {
            flash.error("Host/IP, username, and password are required.");
            Redirect::to("/admin/server_links").into_response()
        } else {
            let link_json = serde_json::json!({
                "host_or_ip": host_or_ip,
            });
            match mk_lib_database::mk_lib_database_link_server::mk_lib_database_link_insert(
                &state.sqlx_pool_rw,
                host_or_ip.to_string(),
                username.to_string(),
                password.to_string(),
                link_json,
            )
            .await
            {
                Ok(_) => Redirect::to("/admin/server_links").into_response(),
                Err(_) => {
                    flash.error("Unable to save the linked server.");
                    Redirect::to("/admin/server_links").into_response()
                }
            }
        }
    }
}
