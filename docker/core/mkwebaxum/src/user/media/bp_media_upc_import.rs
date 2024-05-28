use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_metadata;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_upc_import.html")]
struct TemplateMediaUPCContext {
    template_data: serde_json::Value,
}

pub async fn user_media_upc_import(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = TemplateMediaMovieDetailContext {
            template_data: json!({}),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Deserialize)]
pub struct UPCInput {
    upc_code: String,
    media_type: String,
}

pub async fn user_media_upc_import_post(
    Extension(sqlx_pool): Extension<PgPool>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<UPCInput>,
) -> Redirect {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        // See if the barcode is on the DB
        let upc_exits: bool =
            mk_lib_database::mk_lib_database_metadata_upc::mk_lib_database_metadata_exists_upc(
                &sqlx_pool,
                input_data.upc_code,
            );
        if upc_exits == true {
            // See if the user owns the barcode scanned
            let user_owns: bool =
            mk_lib_database::mk_lib_database_metadata_upc::mk_lib_database_metadata_exists_upc_own(
                &sqlx_pool,
                input_data.upc_code,
                input_data.user_id,
            );
            if user_owns == true {
                // TODO throw owned error
            } else {
                // TODO insert row into DB for user
            }
        } else {
            // Lookup barcodespider
            let json_data: serde_json::Value =
                mk_lib_metadata::provider::barcodespider::provider_barcodespider_fetch(
                    input_data.upc_code,
                );
            if json_data["Status"] == "404" {
                // Lookup upcitemdb since not on barcodespider
                let json_data: serde_json::Value =
                    mk_lib_metadata::provider::upcitemdb::provider_upcitemdb_fetch(
                        input_data.upc_code,
                    );
                    if json_data["Status"] == "404" {
                        // TODO throw not found error
                    }
                }
        }
    }
    Redirect::to("/user/home")
}
