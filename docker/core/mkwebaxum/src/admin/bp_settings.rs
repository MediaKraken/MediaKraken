use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::{Form, Query, State};
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_settings.html")]
struct AdminSettingsTemplate<'a> {
    page_title: Option<String>,
    template_saved_message: &'a String,
    template_server_name: &'a String,
    template_server_motd: &'a String,
    template_server_purge_activity: &'a String,
    template_server_lock_account: &'a String,
    template_metadata_with_media: &'a bool,
    template_subtitle_for_media: &'a bool,
    template_subtitle_opensubtitles: &'a bool,
    template_subtitle_skip_if_audio: &'a bool,
    template_docker_mumble: &'a bool,
    template_docker_teamspeak3: &'a bool,
    template_docker_transmission: &'a bool,
}

/// Query params used to show a save result banner after a POST redirect.
#[derive(Deserialize)]
pub struct SettingsPageQuery {
    #[serde(default)]
    saved: Option<i64>,
    #[serde(default)]
    error: Option<i64>,
}

/// Submitted settings form. Checkboxes arrive as the string "on" when
/// ticked and are simply absent when not ticked.
#[derive(Deserialize)]
pub struct SettingsForm {
    #[serde(rename = "server.name", default)]
    server_name: Option<String>,
    #[serde(rename = "server.motd", default)]
    server_motd: Option<String>,
    #[serde(rename = "server.purge_activity", default)]
    server_purge_activity: Option<String>,
    #[serde(rename = "server.lock_account", default)]
    server_lock_account: Option<String>,
    #[serde(rename = "server.metadata_with_media")]
    metadata_with_media: Option<String>,
    #[serde(rename = "server.subtitle_for_media")]
    subtitle_for_media: Option<String>,
    #[serde(rename = "server.subtitle_opensubtitles")]
    subtitle_opensubtitles: Option<String>,
    #[serde(rename = "server.subtitle_skip_if_audio")]
    subtitle_skip_if_audio: Option<String>,
    #[serde(rename = "server.docker_mumble")]
    docker_mumble: Option<String>,
    #[serde(rename = "server.docker_teamspeak3")]
    docker_teamspeak3: Option<String>,
    #[serde(rename = "server.docker_transmission")]
    docker_transmission: Option<String>,
}

/// Walk (creating as needed) to a nested JSON object path and set the
/// final key to `value`.
fn set_json_value(root: &mut serde_json::Value, path: &[&str], value: serde_json::Value) {
    let mut current = root;
    for key in path {
        if !current.is_object() {
            *current = serde_json::Value::Object(serde_json::Map::new());
        }
        let map = match current.as_object_mut() {
            Some(map) => map,
            None => return,
        };
        current = map
            .entry(key.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    }
    *current = value;
}

/// Read a nested JSON path as a String (numbers are stringified; null and
/// missing values become an empty string).
fn get_json_string(root: &serde_json::Value, path: &[&str]) -> String {
    let mut current = root;
    for key in path {
        current = match current.get(*key) {
            Some(value) => value,
            None => return String::new(),
        };
    }
    match current {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        _ => String::new(),
    }
}

fn get_json_bool(root: &serde_json::Value, path: &[&str]) -> bool {
    let mut current = root;
    for key in path {
        current = match current.get(*key) {
            Some(value) => value,
            None => return false,
        };
    }
    current.as_bool().unwrap_or(false)
}

/// Numeric settings are stored as integers (or null when blank).
fn json_number_or_null(text: &str) -> serde_json::Value {
    text.trim()
        .parse::<i64>()
        .map(serde_json::Value::from)
        .unwrap_or(serde_json::Value::Null)
}

async fn render_settings(state: &AppState, saved_message: &String) -> (StatusCode, Html<String>) {
    // Load current options; if the read fails we still show the page with
    // blank values rather than a hard error.
    let option_json =
        match mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(
            &state.sqlx_pool_ro,
        )
        .await
        {
            Ok(json) => json,
            Err(error) => {
                eprintln!("Failed to read mm_options_json for settings page: {error}");
                serde_json::json!({})
            }
        };
    let server_name = get_json_string(&option_json, &["MediaKrakenServer", "Server Name"]);
    let server_motd = get_json_string(&option_json, &["MediaKrakenServer", "MOTD"]);
    let server_purge_activity = get_json_string(&option_json, &["User", "Activity Purge"]);
    let server_lock_account = get_json_string(&option_json, &["User", "Password Lock"]);
    let metadata_with_media = get_json_bool(&option_json, &["Metadata", "MetadataImageLocal"]);
    let subtitle_for_media = get_json_bool(&option_json, &["Metadata", "DL Subtitle"]);
    let subtitle_opensubtitles = get_json_bool(&option_json, &["Metadata", "OpenSubtitles"]);
    let subtitle_skip_if_audio = get_json_bool(&option_json, &["Metadata", "SubtitleSkipIfAudio"]);
    let docker_mumble = get_json_bool(&option_json, &["Docker Instances", "mumble"]);
    let docker_teamspeak3 = get_json_bool(&option_json, &["Docker Instances", "teamspeak"]);
    let docker_transmission = get_json_bool(&option_json, &["Docker Instances", "transmission"]);
    let template = AdminSettingsTemplate {
        page_title: Some("MediaKraken Admin Settings".to_string()),
        template_saved_message: saved_message,
        template_server_name: &server_name,
        template_server_motd: &server_motd,
        template_server_purge_activity: &server_purge_activity,
        template_server_lock_account: &server_lock_account,
        template_metadata_with_media: &metadata_with_media,
        template_subtitle_for_media: &subtitle_for_media,
        template_subtitle_opensubtitles: &subtitle_opensubtitles,
        template_subtitle_skip_if_audio: &subtitle_skip_if_audio,
        template_docker_mumble: &docker_mumble,
        template_docker_teamspeak3: &docker_teamspeak3,
        template_docker_transmission: &docker_transmission,
    };
    (
        StatusCode::OK,
        Html(template.render().expect("settings template render failed")),
    )
}

pub async fn admin_settings(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<SettingsPageQuery>,
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
        (StatusCode::FORBIDDEN, Html(reply_html))
    } else {
        let saved_message = match (query.saved, query.error) {
            (Some(_), _) => "Settings saved.".to_string(),
            (_, Some(_)) => "Settings could not be saved. Please try again.".to_string(),
            _ => String::new(),
        };
        render_settings(&state, &saved_message).await
    }
}

pub async fn admin_settings_post(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<SettingsForm>,
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

    // Merge the submitted values into the existing options JSON so every
    // key not covered by the form (API keys, Trakt, etc.) is preserved.
    let mut option_json =
        match mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(
            &state.sqlx_pool_ro,
        )
        .await
        {
            Ok(json) => json,
            Err(error) => {
                eprintln!("Failed to read mm_options_json for settings save: {error}");
                return Redirect::to("/admin/settings?error=1").into_response();
            }
        };

    set_json_value(
        &mut option_json,
        &["MediaKrakenServer", "Server Name"],
        serde_json::Value::String(
            input_data
                .server_name
                .clone()
                .unwrap_or_default()
                .trim()
                .to_string(),
        ),
    );
    let motd = input_data
        .server_motd
        .clone()
        .unwrap_or_default()
        .trim()
        .to_string();
    set_json_value(
        &mut option_json,
        &["MediaKrakenServer", "MOTD"],
        if motd.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::Value::String(motd)
        },
    );
    set_json_value(
        &mut option_json,
        &["User", "Activity Purge"],
        json_number_or_null(&input_data.server_purge_activity.clone().unwrap_or_default()),
    );
    set_json_value(
        &mut option_json,
        &["User", "Password Lock"],
        json_number_or_null(&input_data.server_lock_account.clone().unwrap_or_default()),
    );
    set_json_value(
        &mut option_json,
        &["Metadata", "MetadataImageLocal"],
        serde_json::Value::Bool(input_data.metadata_with_media.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Metadata", "DL Subtitle"],
        serde_json::Value::Bool(input_data.subtitle_for_media.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Metadata", "OpenSubtitles"],
        serde_json::Value::Bool(input_data.subtitle_opensubtitles.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Metadata", "SubtitleSkipIfAudio"],
        serde_json::Value::Bool(input_data.subtitle_skip_if_audio.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Docker Instances", "mumble"],
        serde_json::Value::Bool(input_data.docker_mumble.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Docker Instances", "teamspeak"],
        serde_json::Value::Bool(input_data.docker_teamspeak3.is_some()),
    );
    set_json_value(
        &mut option_json,
        &["Docker Instances", "transmission"],
        serde_json::Value::Bool(input_data.docker_transmission.is_some()),
    );

    match mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_update(
        &state.sqlx_pool_rw,
        option_json,
    )
    .await
    {
        Ok(()) => Redirect::to("/admin/settings?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to save mm_options_json: {error}");
            Redirect::to("/admin/settings?error=1").into_response()
        }
    }
}
