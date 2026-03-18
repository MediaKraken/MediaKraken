use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::Form,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Debug, Clone)]
struct PhysicalMediaMatch {
    upc_code: String,
    title: String,
    source: String,
    year: Option<String>,
    media_format: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_upc_import.html")]
struct TemplateMediaUPCContext {
    page_title: Option<String>,
    upc_code: String,
    lookup_error: Option<String>,
    confirmation_message: Option<String>,
    match_result: Option<PhysicalMediaMatch>,
}

#[derive(Deserialize)]
pub struct PhysicalMediaForm {
    #[serde(default)]
    upc_code: String,
    #[serde(default)]
    action: String,
    #[serde(default)]
    match_title: String,
    #[serde(default)]
    match_source: String,
    #[serde(default)]
    match_year: String,
    #[serde(default)]
    match_media_format: String,
}

pub async fn user_media_physical(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    if !has_user_view_rights(method, &auth).await {
        return unauthorized_response();
    }

    render_physical_media_page(TemplateMediaUPCContext {
        page_title: Some("MediaKraken Physical Media Scan".to_string()),
        upc_code: String::new(),
        lookup_error: None,
        confirmation_message: None,
        match_result: None,
    })
}

pub async fn user_media_physical_post(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<PhysicalMediaForm>,
) -> impl IntoResponse {
    if !has_user_view_rights(method, &auth).await {
        return unauthorized_response();
    }

    let mut page_context = TemplateMediaUPCContext {
        page_title: Some("MediaKraken Physical Media Scan".to_string()),
        upc_code: input_data.upc_code.trim().to_string(),
        lookup_error: None,
        confirmation_message: None,
        match_result: rebuild_match_from_form(&input_data),
    };

    match input_data.action.as_str() {
        "lookup" => {
            if !is_valid_upc(&page_context.upc_code) {
                page_context.lookup_error =
                    Some("Enter a valid UPC with 8-14 numeric digits.".to_string());
            } else {
                match lookup_media_by_upc(&page_context.upc_code).await {
                    Ok(Some(found_match)) => {
                        page_context.match_result = Some(found_match);
                    }
                    Ok(None) => {
                        page_context.lookup_error = Some(
                            "No media match was found on eBay or Amazon for that UPC.".to_string(),
                        );
                        page_context.match_result = None;
                    }
                    Err(error_message) => {
                        page_context.lookup_error = Some(error_message);
                        page_context.match_result = None;
                    }
                }
            }
        }
        "confirm" => {
            page_context.confirmation_message = Some("Match accepted for review.".to_string());
        }
        "flag" => {
            page_context.confirmation_message =
                Some("Match flagged as incorrect for review.".to_string());
        }
        _ => {
            page_context.lookup_error = Some("Unknown action requested.".to_string());
        }
    }

    render_physical_media_page(page_context)
}

fn rebuild_match_from_form(input_data: &PhysicalMediaForm) -> Option<PhysicalMediaMatch> {
    if input_data.match_title.trim().is_empty() {
        return None;
    }

    Some(PhysicalMediaMatch {
        upc_code: input_data.upc_code.trim().to_string(),
        title: input_data.match_title.trim().to_string(),
        source: input_data.match_source.trim().to_string(),
        year: optional_string(&input_data.match_year),
        media_format: optional_string(&input_data.match_media_format),
    })
}

fn optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

async fn lookup_media_by_upc(upc_code: &str) -> Result<Option<PhysicalMediaMatch>, String> {
    let ebay_results: Result<Vec<mk_lib_metadata::provider::ebay::EbayMediaResult>, _> =
        mk_lib_metadata::provider::ebay::provider_ebay_fetch_by_upc(upc_code).await;

    match ebay_results {
        Ok(results) => {
            let first_result: Option<mk_lib_metadata::provider::ebay::EbayMediaResult> =
                results.into_iter().next();
            if let Some(result) = first_result {
                return Ok(Some(PhysicalMediaMatch {
                    upc_code: upc_code.to_string(),
                    title: result.title,
                    source: "eBay".to_string(),
                    year: result.year.map(|year: i32| year.to_string()),
                    media_format: result.media_format,
                }));
            }
        }
        Err(_) => {
            // Fallback to Amazon below.
        }
    }

    let amazon_result: Result<Option<mk_lib_metadata::provider::amazon::AmazonUpcResult>, _> =
        mk_lib_metadata::provider::amazon::provider_amazon_search_by_upc(upc_code).await;

    match amazon_result {
        Ok(Some(result)) => Ok(Some(PhysicalMediaMatch {
            upc_code: upc_code.to_string(),
            title: result.title,
            source: "Amazon".to_string(),
            year: result.year.map(|year: u16| year.to_string()),
            media_format: result.media_format,
        })),
        Ok(None) => Ok(None),
        Err(_) => Err("Unable to search eBay/Amazon right now. Please try again.".to_string()),
    }
}

async fn has_user_view_rights(
    method: Method,
    auth: &AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> bool {
    let current_user = auth.current_user.clone().unwrap_or_default();
    Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
        .requires(Rights::any([Rights::permission("User::View")]))
        .validate(&current_user, &method, None)
        .await
}

fn unauthorized_response() -> (StatusCode, axum::response::Response) {
    let template = TemplateError401Context {};
    match template.render() {
        Ok(reply_html) => (StatusCode::UNAUTHORIZED, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Html("Unauthorized".to_string()).into_response(),
        ),
    }
}

fn render_physical_media_page(
    template_context: TemplateMediaUPCContext,
) -> (StatusCode, axum::response::Response) {
    match template_context.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Failed to render the physical media page.".to_string()).into_response(),
        ),
    }
}

fn is_valid_upc(upc_code: &str) -> bool {
    let trimmed = upc_code.trim();
    let length = trimmed.chars().count();
    (8..=14).contains(&length) && trimmed.chars().all(|character| character.is_ascii_digit())
}
