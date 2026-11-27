use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::{Query, State};
use axum::{
    extract::Form,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

/// Barcode type stored for admin-imported UPC codes. No type picker exists
/// in the import form, so all imported codes get this default.
const BAR_CODE_TYPE_IMPORTED_UPC: i16 = 0;

/// Maximum number of codes accepted per import (matches the template limit).
const UPC_IMPORT_MAX_CODES: usize = 500;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_upc_import.html")]
struct TemplateMediaUPCContext<'a> {
    template_import_message: &'a String,
    page_title: Option<String>,
}

/// Query string returned after a POST import so the page can show a summary.
#[derive(Deserialize)]
pub struct UpcImportSummary {
    imported: Option<i64>,
    skipped: Option<i64>,
    failed: Option<i64>,
}

pub async fn admin_upc_import(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(summary): Query<UpcImportSummary>,
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
        let template = TemplateError403Context { is_admin: true };
        let reply_html = template.render().unwrap();
        (StatusCode::FORBIDDEN, Html(reply_html).into_response())
    } else {
        let import_message = match (summary.imported, summary.skipped, summary.failed) {
            (Some(imported), Some(skipped), Some(failed)) => format!(
                "Import complete: {} added, {} already existed, {} failed.",
                imported, skipped, failed
            ),
            _ => String::new(),
        };
        let _ = &state; // state only used by the POST handler
        let template = TemplateMediaUPCContext {
            template_import_message: &import_message,
            page_title: Some("MediaKraken Admin UPC Import".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Deserialize)]
pub struct UPCInput {
    upc_codes: String,
}

pub async fn admin_upc_import_post(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<UPCInput>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET, Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context { is_admin: true };
        let reply_html = template.render().unwrap();
        return (StatusCode::FORBIDDEN, Html(reply_html)).into_response();
    }

    // Accept one code per line (scanner keystrokes, pasted lists, or an
    // uploaded .txt file); tolerate any other separator.
    let codes: Vec<String> = input_data
        .upc_codes
        .split(|c: char| !c.is_ascii_digit())
        .filter(|token| token.len() >= 5 && token.len() <= 14)
        .map(str::to_string)
        .collect();
    let mut unique_codes: Vec<String> = Vec::new();
    for code in codes.iter().take(UPC_IMPORT_MAX_CODES) {
        if !unique_codes.contains(code) {
            unique_codes.push(code.clone());
        }
    }

    let mut imported: i64 = 0;
    let mut skipped: i64 = 0;
    let mut failed: i64 = 0;
    for code in &unique_codes {
        let exists =
            mk_lib_database::database_metadata::mk_lib_database_metadata_upc::mk_lib_database_metadata_exists_upc(
                &state.sqlx_pool_ro,
                code,
            )
            .await;
        match exists {
            Ok(true) => skipped += 1,
            Ok(false) => {
                // Direct insert: the shared lib `mk_lib_database_metadata_upc_insert`
                // has a missing comma in its SQL and would fail at runtime.
                let new_guid = uuid::Uuid::now_v7();
                let result = sqlx::query(
                    r#"insert into mm_bar_codes (mm_bar_code_uuid, mm_bar_code_code, mm_bar_code_type, mm_bar_code_json)
                       values ($1, $2, $3, $4)"#,
                )
                .bind(new_guid)
                .bind(code)
                .bind(BAR_CODE_TYPE_IMPORTED_UPC)
                .bind(serde_json::json!({
                    "Source": "admin_import"
                }))
                .execute(&state.sqlx_pool_rw)
                .await;
                match result {
                    Ok(_) => imported += 1,
                    Err(error) => {
                        eprintln!("UPC import insert failed for {code}: {error}");
                        failed += 1;
                    }
                }
            }
            Err(error) => {
                eprintln!("UPC import lookup failed for {code}: {error}");
                failed += 1;
            }
        }
    }

    Redirect::to(&format!(
        "/admin/upc?imported={imported}&skipped={skipped}&failed={failed}"
    ))
    .into_response()
}
