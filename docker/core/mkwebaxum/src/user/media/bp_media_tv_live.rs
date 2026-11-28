use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::State;
use axum::{
    extract::Query,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::Value;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

use mk_lib_database::database_metadata::mk_lib_database_metadata_tv_live::MetaTVStationList;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

/// One program slot positioned on the day grid (percentages of the 24h axis).
struct EpgProgram {
    title: String,
    start_label: String,
    left_pct: String,
    width_pct: String,
    is_live: bool,
}

/// One station row of the grid.
struct EpgStationRow {
    station_channel: String,
    station_name: String,
    programs: Vec<EpgProgram>,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_tv_live.html")]
struct TemplateMediaTVLiveContext {
    template_data_rows: Vec<EpgStationRow>,
    /// `(hour label, left percentage of the 24h axis)` pairs for the ruler.
    template_data_hours: Vec<(String, String)>,
    template_data_date_label: String,
    template_data_prev_date: Option<String>,
    template_data_next_date: Option<String>,
    template_data_now_pct: Option<String>,
    template_data_has_stations: bool,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct LiveTvQuery {
    date: Option<String>,
}

/// One program slot parsed out of a station's daily schedule JSON.
struct ParsedProgram {
    program_id: String,
    start_minutes: i64,
    duration_seconds: i64,
}

const MINUTES_PER_DAY: i64 = 24 * 60;

/// Collect the program objects from a stored daily schedule JSON. The value
/// may be the array of programs itself, an object wrapping them under
/// "programs", or a single program object.
fn parse_schedule_json(schedule_json: &Value) -> Vec<ParsedProgram> {
    let program_values: Vec<&Value> = match schedule_json {
        Value::Array(items) => items.iter().collect(),
        Value::Object(_) => {
            if let Some(Value::Array(items)) = schedule_json.get("programs") {
                items.iter().collect()
            } else if schedule_json.get("airDateTime").is_some() {
                vec![schedule_json]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };

    let mut programs = Vec::new();
    for value in program_values {
        let Some(obj) = value.as_object() else {
            continue;
        };
        let Some(air_str) = obj.get("airDateTime").and_then(Value::as_str) else {
            continue;
        };
        let Ok(start) = DateTime::parse_from_rfc3339(air_str) else {
            continue;
        };
        let utc = start.with_timezone(&Utc);
        let Some(midnight) = utc.date_naive().and_hms_opt(0, 0, 0) else {
            continue;
        };
        let start_minutes = (utc.naive_utc() - midnight).num_minutes();
        if !(0..MINUTES_PER_DAY).contains(&start_minutes) {
            continue;
        }
        programs.push(ParsedProgram {
            program_id: obj
                .get("programID")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            start_minutes,
            duration_seconds: obj.get("duration").and_then(Value::as_i64).unwrap_or(0),
        });
    }
    programs
}

/// Prefer the English title120 from the SD program detail, then any title.
fn program_title(program_json: &Value) -> Option<String> {
    let titles = program_json.get("titles")?.as_array()?;
    let mut fallback = None;
    for entry in titles {
        let Some(title) = entry.get("title120").and_then(Value::as_str) else {
            continue;
        };
        let language = entry
            .get("titleLanguage")
            .and_then(Value::as_str)
            .unwrap_or("");
        if language == "en" {
            return Some(title.to_string());
        }
        if fallback.is_none() {
            fallback = Some(title.to_string());
        }
    }
    fallback
}

/// Sort stations by numeric channel number first, then channel string and
/// name, so "2" comes before "10".
fn channel_sort_key(channel: &str) -> (u64, String) {
    let numeric: u64 = channel
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    (numeric, channel.to_string())
}

fn shift_date(date: NaiveDate, days: i64) -> Option<NaiveDate> {
    if days >= 0 {
        date.checked_add_days(chrono::Days::new(days as u64))
    } else {
        date.checked_sub_days(chrono::Days::new((-days) as u64))
    }
}

fn render_error_401() -> (StatusCode, Html<String>) {
    let template = TemplateError401Context {};
    match template.render() {
        Ok(html) => (StatusCode::UNAUTHORIZED, Html(html)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Internal server error".to_string()),
        ),
    }
}

async fn build_live_tv_response(
    state: &AppState,
    requested_date: Option<&str>,
) -> (StatusCode, Html<String>) {
    let pool = &state.sqlx_pool_ro;
    let today = Utc::now().date_naive();
    let selected = match requested_date.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    {
        Some(date) => date,
        None => today,
    };

    let (stations_result, day_result, programs_result, range_result) = tokio::join!(
        mk_lib_database::database_metadata::mk_lib_database_metadata_tv_live::
            mk_lib_database_meta_tv_live_station_read(pool),
        mk_lib_database::database_metadata::mk_lib_database_metadata_tv_live::
            mk_lib_database_meta_tv_live_day_read(pool, selected),
        mk_lib_database::database_metadata::mk_lib_database_metadata_tv_live::
            mk_lib_database_meta_tv_schedule_program_read(pool),
        mk_lib_database::database_metadata::mk_lib_database_metadata_tv_live::
            mk_lib_database_meta_tv_schedule_date_range(pool),
    );

    if let Err(error) = &stations_result {
        tracing::error!(%error, "live tv station read failed");
    }
    if let Err(error) = &day_result {
        tracing::error!(%error, "live tv day read failed");
    }
    if let Err(error) = &programs_result {
        tracing::error!(%error, "live tv program read failed");
    }
    if let Err(error) = &range_result {
        tracing::error!(%error, "live tv schedule date range failed");
    }

    let (stations, day_rows, program_list, date_range) =
        match (stations_result, day_result, programs_result, range_result) {
            (Ok(stations), Ok(day_rows), Ok(program_list), Ok(date_range)) => {
                (stations, day_rows, program_list, date_range)
            }
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("Internal server error".to_string()),
                );
            }
        };

    // programID -> display title
    let mut title_map: HashMap<String, String> = HashMap::new();
    for program in &program_list {
        if let Some(title) = program_title(&program.mm_tv_schedule_program_json) {
            title_map.insert(program.mm_tv_schedule_program_id.clone(), title);
        }
    }

    // (station name, channel) -> daily schedule JSON
    let day_by_station: HashMap<(String, String), &Value> = day_rows
        .iter()
        .map(|row| {
            (
                (
                    row.mm_tv_station_name.clone(),
                    row.mm_tv_station_channel.clone(),
                ),
                &row.mm_tv_schedule_json,
            )
        })
        .collect();

    let now = Utc::now();
    let now_is_selected_day = selected == today;
    let now_minutes: Option<i64> = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .map(|midnight| (now.naive_utc() - midnight).num_minutes());

    let now_pct = if now_is_selected_day {
        now_minutes.map(|minutes| format!("{:.2}", minutes as f64 * 100.0 / 1440.0))
    } else {
        None
    };

    let mut stations: Vec<MetaTVStationList> = stations;
    stations.sort_by(|a, b| {
        channel_sort_key(&a.mm_tv_station_channel)
            .cmp(&channel_sort_key(&b.mm_tv_station_channel))
            .then_with(|| a.mm_tv_station_name.cmp(&b.mm_tv_station_name))
    });

    let rows: Vec<EpgStationRow> = stations
        .iter()
        .map(|station| {
            let schedule_json = day_by_station.get(&(
                station.mm_tv_station_name.clone(),
                station.mm_tv_station_channel.clone(),
            ));
            let programs: Vec<EpgProgram> = schedule_json
                .map(|json| parse_schedule_json(json))
                .unwrap_or_default()
                .into_iter()
                .map(|program| {
                    let duration = if program.duration_seconds > 0 {
                        program.duration_seconds
                    } else {
                        900 // keep slots without a duration visible
                    };
                    let left_pct = program.start_minutes as f64 * 100.0 / 1440.0;
                    let width_pct = (duration as f64 * 100.0 / 86400.0).min(100.0 - left_pct);
                    let start_seconds = program.start_minutes * 60;
                    let start_label = format!(
                        "{:02}:{:02}",
                        (start_seconds / 3600) % 24,
                        (start_seconds / 60) % 60
                    );
                    let is_live = now_is_selected_day
                        && now_minutes.is_some_and(|minutes| {
                            minutes >= program.start_minutes
                                && minutes < program.start_minutes + (duration / 60).max(1)
                        });
                    EpgProgram {
                        title: title_map
                            .get(&program.program_id)
                            .cloned()
                            .unwrap_or_else(|| program.program_id.clone()),
                        start_label,
                        left_pct: format!("{left_pct:.2}"),
                        width_pct: format!("{width_pct:.2}"),
                        is_live,
                    }
                })
                .collect();
            EpgStationRow {
                station_channel: station.mm_tv_station_channel.clone(),
                station_name: station.mm_tv_station_name.clone(),
                programs,
            }
        })
        .collect();

    let (prev_date, next_date) = match date_range {
        Some((min, max)) => (
            if selected > min {
                shift_date(selected, -1).map(|d| d.to_string())
            } else {
                None
            },
            if selected < max {
                shift_date(selected, 1).map(|d| d.to_string())
            } else {
                None
            },
        ),
        None => (None, None),
    };

    let hours: Vec<(String, String)> = (0..24)
        .map(|hour| {
            (
                format!("{hour:02}:00"),
                format!("{:.4}", hour as f64 * 100.0 / 24.0),
            )
        })
        .collect();

    let template = TemplateMediaTVLiveContext {
        template_data_rows: rows,
        template_data_hours: hours,
        template_data_date_label: selected.format("%A, %B %d, %Y").to_string(),
        template_data_prev_date: prev_date,
        template_data_next_date: next_date,
        template_data_now_pct: now_pct,
        template_data_has_stations: !stations.is_empty(),
        page_title: Some(format!(
            "MediaKraken Live TV - {}",
            selected.format("%Y-%m-%d")
        )),
    };

    match template.render() {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Internal server error".to_string()),
        ),
    }
}

pub async fn user_media_tv_live(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<LiveTvQuery>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    let authorized = Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await;

    if !authorized {
        render_error_401().into_response()
    } else {
        build_live_tv_response(&state, query.date.as_deref())
            .await
            .into_response()
    }
}
