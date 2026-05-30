use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, Path, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

const ALLOWED_CRON_SCHEDULE_TYPES: [&str; 4] = ["Week(s)", "Day(s)", "Hour(s)", "Minute(s)"];

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_cron.html")]
struct TemplateCronContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct CronUpdateInput {
    guid: uuid::Uuid,
    name: String,
    description: String,
    enabled: Option<String>,
    schedule_type: String,
    schedule_time: i16,
    cron_json: String,
}

fn validate_cron_update(
    input_data: &CronUpdateInput,
) -> Result<(String, String, bool, String, i16, serde_json::Value), &'static str> {
    let cron_name = input_data.name.trim();
    if cron_name.is_empty() {
        return Err("Cron name is required.");
    }

    let schedule_type = input_data.schedule_type.trim();
    if !ALLOWED_CRON_SCHEDULE_TYPES.contains(&schedule_type) {
        return Err("Invalid schedule type.");
    }

    if input_data.schedule_time < 1 {
        return Err("Schedule time must be at least 1.");
    }

    let cron_json: serde_json::Value = serde_json::from_str(input_data.cron_json.trim())
        .map_err(|_| "Cron JSON must be valid JSON.")?;

    if cron_json
        .get("route_key")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .is_none()
    {
        return Err("Cron JSON must include a non-empty route_key.");
    }

    Ok((
        cron_name.to_string(),
        input_data.description.trim().to_string(),
        input_data.enabled.is_some(),
        schedule_type.to_string(),
        input_data.schedule_time,
        cron_json,
    ))
}

async fn user_can_view_admin_cron(
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

pub async fn admin_cron(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    if !user_can_view_admin_cron(&method, &auth).await {
        let template = TemplateError403Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let cron_list = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(
            &state.sqlx_pool_ro,
        )
        .await
        ?;
        let cron_data = !cron_list.is_empty();
        let template = TemplateCronContext {
            template_data: &cron_list,
            template_data_exists: &cron_data,
            page_title: Some("MediaKraken Admin Cron".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn admin_cron_run(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    if !user_can_view_admin_cron(&method, &auth).await {
        Redirect::to("/error/403")
    } else {
        let row_data = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_json(
            &state.sqlx_pool_rw,
            guid,
        )
        .await
        ?;
        let (rabbit_connection, rabbit_channel) =
            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp")
                .await
                ?;
        let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            row_data["route_key"].as_str().unwrap_or(""),
            row_data.to_string(),
        )
        .await
        ?;
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            ?;
        let _result = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
            &state.sqlx_pool_rw,
            guid,
        )
        .await
        ?;
        Redirect::to("/admin/cron")
    }
}

pub async fn admin_cron_update(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<CronUpdateInput>,
) -> impl IntoResponse {
    if !user_can_view_admin_cron(&method, &auth).await {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let (cron_name, cron_description, cron_enabled, schedule_type, schedule_time, cron_json) =
        match validate_cron_update(&input_data) {
            Ok(validated) => validated,
            Err(message) => return (StatusCode::BAD_REQUEST, message).into_response(),
        };

    let update_result = sqlx::query(
        r#"
        update mm_cron_jobs
        set
            mm_cron_name = $1,
            mm_cron_description = $2,
            mm_cron_enabled = $3,
            mm_cron_schedule_type = $4,
            mm_cron_schedule_time = $5,
            mm_cron_json = $6
        where mm_cron_guid = $7
        "#,
    )
    .bind(cron_name)
    .bind(cron_description)
    .bind(cron_enabled)
    .bind(schedule_type)
    .bind(schedule_time)
    .bind(cron_json)
    .bind(input_data.guid)
    .execute(&state.sqlx_pool_rw)
    .await;

    match update_result {
        Ok(result) if result.rows_affected() == 1 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Cron entry not found.").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to update cron entry.",
        )
            .into_response(),
    }
}
