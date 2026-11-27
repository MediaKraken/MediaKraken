use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::State;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware.html")]
struct TemplateUserHardwareContext<'a> {
    template_data_phue_exists: &'a bool,
    page_title: Option<String>,
}

pub async fn user_hardware(
    State(_state): State<AppState>,
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
        let phue_exists: bool = true;
        let template = TemplateUserHardwareContext {
            template_data_phue_exists: &phue_exists,
            page_title: Some("MediaKraken Hardware".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_phue.html")]
struct TemplateUserHardwarePhueContext {
    template_data_phue: i32,
    page_title: Option<String>,
}

pub async fn user_hardware_phue(
    State(_state): State<AppState>,
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
        let template = TemplateUserHardwarePhueContext {
            template_data_phue: 0,
            page_title: Some("MediaKraken Hardware".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/// One discovered Chromecast: (hostname, ip:port).
type ChromecastRow = (String, String);

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_chromecast.html")]
struct TemplateUserHardwareChromecastContext<'a> {
    template_data_devices: &'a Vec<ChromecastRow>,
    template_data_note: &'a String,
    page_title: Option<String>,
}

/// Live mDNS scan for Chromecast devices. The library listens for a fixed
/// 15 second window, so this page intentionally takes up to ~15s; the
/// template tells the user it is a network scan.
pub async fn user_hardware_chromecast(
    State(_state): State<AppState>,
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
        let mut note = String::new();
        let devices: Vec<ChromecastRow> =
            match mk_lib_hardware::mk_lib_hardware_chromecast::mk_hardware_chromecast_discover()
                .await
            {
                Ok(found) => found.into_iter().map(|d| (d.hostname, d.address)).collect(),
                Err(error) => {
                    eprintln!("Chromecast discovery failed: {error}");
                    note = "Chromecast discovery failed; multicast may be unavailable in this network.".to_string();
                    Vec::new()
                }
            };

        let template = TemplateUserHardwareChromecastContext {
            template_data_devices: &devices,
            template_data_note: &note,
            page_title: Some("MediaKraken Hardware - Chromecast".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/// One discovered HDHomeRun channel: (guide number, guide name, stream url).
type TvtunerChannelRow = (String, String, String);

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_tvtuner.html")]
struct TemplateUserHardwareTvtunerContext<'a> {
    template_data_tuners: &'a Vec<String>,
    template_data_channels: &'a Vec<TvtunerChannelRow>,
    template_data_note: &'a String,
    page_title: Option<String>,
}

/// Live SSDP scan for HDHomeRun tuners, followed by a per-tuner lineup fetch
/// (capped so unresponsive tuners cannot stall the page).
pub async fn user_hardware_tvtuner(
    State(_state): State<AppState>,
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
        let mut note = String::new();
        let tuners: Vec<String> =
            mk_lib_hardware::mk_lib_hardware_hdhomerun::mk_lib_hardware_hdhomerun_discover()
                .await
                .into_iter()
                .map(|location| location.to_string())
                .collect();

        let mut channels: Vec<TvtunerChannelRow> = Vec::new();
        if !tuners.is_empty() {
            match timeout(
                Duration::from_secs(15),
                mk_lib_hardware::mk_lib_hardware_hdhomerun::mk_lib_hardware_hdhomerun_channel_discover(),
            )
            .await
            {
                Ok(found) => {
                    channels = found
                        .into_iter()
                        .map(|c| (c.guide_number, c.guide_name, c.stream_url))
                        .collect()
                }
                Err(_) => {
                    note = "Fetching the channel lineups timed out.".to_string();
                }
            }
        }

        let template = TemplateUserHardwareTvtunerContext {
            template_data_tuners: &tuners,
            template_data_channels: &channels,
            template_data_note: &note,
            page_title: Some("MediaKraken Hardware - TV Tuner".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_globalcache.html")]
struct TemplateUserHardwareGlobalcacheContext<'a> {
    template_data_devices: &'a Vec<mk_lib_database::mk_lib_database_hardware_device::DBDeviceList>,
    template_data_total: usize,
    template_data_note: &'a String,
    page_title: Option<String>,
}

/// The GlobalCache hardware catalog (mm_hardware_model), imported by the
/// mkglobalcache app. Capped client-side so a large import cannot stall the
/// page render.
pub async fn user_hardware_globalcache(
    State(state): State<AppState>,
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
        const MAX_ROWS: usize = 200;
        let mut note = String::new();
        let all_devices =
            match mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_device_read(
                &state.sqlx_pool_ro,
            )
            .await
            {
                Ok(rows) => rows,
                Err(error) => {
                    eprintln!("GlobalCache hardware read failed: {error}");
                    note = "Reading the GlobalCache hardware database failed.".to_string();
                    Vec::new()
                }
            };
        let total = all_devices.len();
        let devices: Vec<mk_lib_database::mk_lib_database_hardware_device::DBDeviceList> =
            all_devices.into_iter().take(MAX_ROWS).collect();

        let template = TemplateUserHardwareGlobalcacheContext {
            template_data_devices: &devices,
            template_data_total: total,
            template_data_note: &note,
            page_title: Some("MediaKraken Hardware - GlobalCache".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_dlna.html")]
struct TemplateUserHardwareDlnaContext {
    page_title: Option<String>,
}

/// DLNA renderer discovery is not wired up yet: the library's discover
/// function logs findings instead of returning them, and no database table
/// stores discovered renderers. The page shows that state honestly.
pub async fn user_hardware_dlna(
    State(_state): State<AppState>,
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
        let template = TemplateUserHardwareDlnaContext {
            page_title: Some("MediaKraken Hardware - DLNA".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_onvif.html")]
struct TemplateUserHardwareOnvifContext {
    page_title: Option<String>,
}

/// OnVif camera discovery is not wired up yet: the library's onvif module is
/// not compiled in the published crate. The page shows that state honestly.
pub async fn user_hardware_onvif(
    State(_state): State<AppState>,
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
        let template = TemplateUserHardwareOnvifContext {
            page_title: Some("MediaKraken Hardware - OnVif Cameras".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
