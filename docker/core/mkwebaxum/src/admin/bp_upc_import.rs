use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::Form,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_sqlx::{SessionPgPool};
use axum_session_auth::*;
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_metadata;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use axum::extract::State;
use crate::AppState;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_upc_import.html")]
struct TemplateMediaUPCContext {
    template_data: serde_json::Value,
        page_title: Option<String>,

}

pub async fn admin_upc_import(
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
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = TemplateMediaUPCContext {
            template_data: json!({}),
                        page_title: Some("MediaKraken Admin UPC Import".to_string()),

        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Deserialize)]
pub struct UPCInput {
    upc_code: i32,
    media_type: String,
}

pub async fn admin_upc_import_post(
     State(state): State<AppState>,
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<UPCInput>,
) -> Redirect {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        //(StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        // See if the barcode is on the DB
        let upc_exits: bool =
            mk_lib_database::database_metadata::mk_lib_database_metadata_upc::mk_lib_database_metadata_exists_upc(
              &state.sqlx_pool_ro,
                &input_data.upc_code,
            ) .await
            ?;
        }
    }
    Redirect::to("/admin/home")
}
