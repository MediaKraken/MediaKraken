use crate::AppState;
use crate::mk_lib_database;
use mk_lib_share::mk_lib_file_smb::{classify_smbclient_browse_error, is_smb_ls_date};
use askama::Template;
use axum::extract::{Form, State};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::Flash;
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use tokio::process::Command;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_library.html")]
struct TemplateAdminLibraryContext<'a> {
    template_data_share: &'a Vec<mk_lib_database::mk_lib_database_network_share::DBShareList>,
    template_data_libary: &'a Vec<mk_lib_database::mk_lib_database_library::DBLibraryAuditList>,
    template_data_share_user:
        &'a Vec<mk_lib_database::mk_lib_database_network_share::DBShareAuthUserList>,
    template_data_exists: &'a bool,
    page_title: Option<String>,
}

impl TemplateAdminLibraryContext<'_> {
    fn share_user_matches(
        &self,
        share: &mk_lib_database::mk_lib_database_network_share::DBShareList,
        auth_user: &mk_lib_database::mk_lib_database_network_share::DBShareAuthUserList,
    ) -> bool {
        matches!(
            share.mm_share_auth_user.as_deref(),
            Some(user) if user == auth_user.mm_share_auth_user
        )
    }
}

#[derive(Deserialize)]
pub struct AddShareLibraryInput {
    share_guid: uuid::Uuid,
    subdirectory: String,
    media_class: i16,
}

#[derive(Deserialize)]
pub struct ShareDirectoryBrowseQuery {
    share_guid: uuid::Uuid,
    path: Option<String>,
}

#[derive(Serialize)]
pub struct ShareDirectoryBrowseResponse {
    current_path: String,
    parent_path: Option<String>,
    directories: Vec<String>,
}

pub async fn admin_library(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Admin library request received",
        "module": module_path!(),
        "function": "admin_library",
        "payload": {},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
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
        let share_list =
            mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_read(
                &state.sqlx_pool_ro,
            )
            .await
            ?;
        let library_list =
            mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_audit_read(
                &state.sqlx_pool_ro,
            )
            .await
            ?;
        let share_user_list =
        mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_user_read(
           &state.sqlx_pool_ro,
        )
        .await
        ?;
        let mut template_data_exists: bool = false;
        if library_list.len() > 0 {
            template_data_exists = true;
        }
        let template = TemplateAdminLibraryContext {
            template_data_share: &share_list,
            template_data_libary: &library_list,
            template_data_share_user: &share_user_list,
            template_data_exists: &template_data_exists,
            page_title: Some("MediaKraken Admin Library".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn admin_library_media_scan(
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
        Redirect::to("/error/403")
    } else {
        let (rabbit_connection, rabbit_channel) =
            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp")
                .await
                ?;
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mkmediascanner",
            json!({"Type": "Library Scan"}).to_string(),
        )
        .await
        ?;
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            ?;
        Redirect::to("/admin/library")
    }
}

pub async fn admin_library_share_add(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    mut flash: Flash,
    Form(input_data): Form<AddShareLibraryInput>,
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
        Redirect::to("/error/403")
    } else {
        let subdirectory = input_data.subdirectory.trim().trim_matches('/');
        if subdirectory.is_empty() {
            flash.error("Directory is required.");
            return Redirect::to("/admin/library");
        }

        let _share_info = match mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
            &state.sqlx_pool_ro,
            input_data.share_guid,
        )
        .await
        {
            Ok(data) => data,
            Err(_) => {
                flash.error("Unable to read share information.");
                return Redirect::to("/admin/library");
            }
        };

        let library_path = subdirectory.to_string();

        match mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_exists(
            &state.sqlx_pool_ro,
            &library_path,
        )
        .await
        {
            Ok(true) => {
                flash.error("Path already exists in library.");
                Redirect::to("/admin/library")
            }
            Ok(false) => {
                match mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_insert(
                    &state.sqlx_pool_rw,
                    &library_path,
                    input_data.media_class,
                    input_data.share_guid,
                )
                .await
                {
                    Ok(_) => Redirect::to("/admin/library"),
                    Err(_) => {
                        flash.error("Unable to add library path.");
                        Redirect::to("/admin/library")
                    }
                }
            }
            Err(_) => {
                flash.error("Unable to validate library path.");
                Redirect::to("/admin/library")
            }
        }
    }
}

pub async fn admin_library_share_scan(
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
        Redirect::to("/error/403")
    } else {
        let (rabbit_connection, rabbit_channel) =
            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp")
                .await
                ?;
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mksharescanner",
            json!({"Type": "Share Scan", "Data": "192.168.1"}).to_string(),
        )
        .await
        ?;
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            ?;
        Redirect::to("/admin/library")
    }
}

pub async fn admin_library_share_directories(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<ShareDirectoryBrowseQuery>,
) -> impl IntoResponse {
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Browsing share directories request received",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Not authorized"})),
        );
    }
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Share directory request authorized",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    let share_info =
        match mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
            &state.sqlx_pool_ro,
            query.share_guid,
        )
        .await
        {
            Ok(data) => data,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Share information not found"})),
                );
            }
        };
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Loaded share details for directory browse",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {"share_guid": query.share_guid.to_string()},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    let requested_path = query.path.unwrap_or_default();
    let cleaned_path = requested_path
        .trim()
        .replace('\\', "/")
        .trim_matches('/')
        .to_string();
    if cleaned_path.split('/').any(|segment| segment == "..") {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid path"})),
        );
    }
    // The path becomes part of an smbclient `-c` script (commands joined with
    // `;`, paths quoted with `"`). Reject characters that could break out of
    // the quoted `cd` argument and inject additional commands.
    if cleaned_path.contains([';', '"', '\n', '\r']) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid path"})),
        );
    }

    let share_name = match mk_lib_database::mk_lib_database_network_share::parse_share_name(
        &share_info.mm_network_share_path,
    ) {
        Some(name) => name,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Share path is not a valid UNC path"})),
            );
        }
    };
    let share_uri = format!("//{}/{}", share_info.mm_network_share_ip, share_name);

    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Loaded share_uri",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {"share_uri": &share_uri},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }

    // Use an in-script `cd` rather than the `-D` flag. Some smbclient builds
    // process `-D` inconsistently when combined with `-c`, leaving the working
    // directory at the share root and returning the same top-level listing
    // regardless of the requested subdirectory. `cd "path"` inside `-c`
    // matches the working pattern in mk_lib_smb::mk_file_smb_client_tree_smbclient.
    //
    // Do NOT prepend `recurse OFF` / `prompt OFF`: in many smbclient builds
    // `recurse` is a no-arg toggle that ignores trailing tokens, so
    // `recurse OFF` actually flips recursion from OFF (the default) to ON.
    // That makes `ls` enumerate every subdirectory in the share and surface
    // them all in the picker regardless of the requested cwd.
    let mut smb_commands: Vec<String> = Vec::new();
    if cleaned_path.is_empty() == false {
        smb_commands.push(format!("cd \"{}\"", cleaned_path));
    }
    smb_commands.push(String::from("ls"));

    let mut smb_command = Command::new("smbclient");
    smb_command
        .arg(share_uri)
        .arg("-g")
        .arg("-c")
        .arg(smb_commands.join(";"));
    if let Some(workgroup) = share_info.mm_network_share_workgroup.as_deref() {
        if workgroup.is_empty() == false {
            smb_command.arg("-W").arg(workgroup);
        }
    }
    if let Some(user) = share_info.mm_share_auth_user.as_deref() {
        let pass = share_info
            .mm_share_auth_password
            .as_deref()
            .unwrap_or_default();
        smb_command.arg("-U").arg(format!("{}%{}", user, pass));
    } else {
        smb_command.arg("-N");
    }
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "Running smbclient directory listing command",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {"command": format!("{:?}", smb_command)},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    let smb_output = match smb_command.output().await {
        Ok(data) => data,
        Err(error) => {
            if let Err(loki_error) =
                mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
                    "level": "error",
                    "message": "smbclient execution failed",
                    "module": module_path!(),
                    "function": "admin_library_share_directories",
                    "payload": {"error": error.to_string()},
                }))
                .await
            {
                eprintln!("loki push error: {loki_error}");
            }
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Unable to run smbclient"})),
            );
        }
    };

    if smb_output.status.success() == false {
        let stdout_output = String::from_utf8_lossy(&smb_output.stdout).to_string();
        let stderr_output = String::from_utf8_lossy(&smb_output.stderr).to_string();
        let details_output = if stderr_output.is_empty() {
            stdout_output.clone()
        } else {
            stderr_output.clone()
        };
        let (status_u16, error_message) =
            classify_smbclient_browse_error(&stdout_output, &stderr_output);
        let status_code =
            StatusCode::from_u16(status_u16).unwrap_or(StatusCode::BAD_GATEWAY);
        if let Err(loki_error) =
            mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
                "level": "error",
                "message": "smbclient failed",
                "module": module_path!(),
                "function": "admin_library_share_directories",
                "payload": {
                    "status_code": smb_output.status.code(),
                    "classified_status": status_u16,
                    "classified_message": error_message,
                    "stdout": stdout_output,
                    "stderr": stderr_output,
                },
            }))
            .await
        {
            eprintln!("loki push error: {loki_error}");
        }
        return (
            status_code,
            Json(json!({"error": error_message, "details": details_output})),
        );
    }

    let stdout_data = String::from_utf8_lossy(&smb_output.stdout).to_string();
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "smbclient directory listing succeeded",
        "module": module_path!(),
        "function": "admin_library_share_directories",
        "payload": {"stdout_len": stdout_data.len()},
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    let mut directories = Vec::new();
    for line in stdout_data.lines() {
        // smbclient's `ls` is column-formatted regardless of `-g` (the grepable
        // flag only changes `-L` share-list output, not file listings). The
        // emitting format string in samba's display_finfo() is:
        //   "  %-30s%7.7s %8.0f  %s"
        //  = 2 spaces, name padded to >=30 cols, 7-col attrs, " ",
        //    8-col size, "  ", 24-col date "Day Mon DD HH:MM:SS YYYY".
        // Some smbclient builds also emit pipe-separated rows
        // (`<type>|<name>|<size>|<date>`); accept both shapes. Parse the
        // column form by trimming fixed-width fields off the right so internal
        // whitespace inside directory names is preserved verbatim.
        if let Some((type_field, rest)) = line.split_once('|') {
            if type_field == "D" {
                let name = rest.split('|').next().unwrap_or_default();
                if !name.is_empty() && name != "." && name != ".." {
                    directories.push(name.to_string());
                    continue;
                }
            }
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        // 2 + 30 + 7 + 1 + 8 + 2 + 24 = 74 minimum bytes.
        if trimmed.len() < 74 {
            continue;
        }
        let date_start = trimmed.len() - 24;
        if !is_smb_ls_date(&trimmed[date_start..]) {
            continue;
        }
        let head = &trimmed[..date_start];
        let head = match head.strip_suffix("  ") {
            Some(rest) => rest,
            None => continue,
        };
        if head.len() < 8 {
            continue;
        }
        let (head, size_field) = head.split_at(head.len() - 8);
        if size_field.trim_start().parse::<u64>().is_err() {
            continue;
        }
        let head = match head.strip_suffix(' ') {
            Some(rest) => rest,
            None => continue,
        };
        if head.len() < 7 {
            continue;
        }
        let (head, attrs_field) = head.split_at(head.len() - 7);
        let attrs = attrs_field.trim_start();
        if attrs.is_empty()
            || !attrs
                .chars()
                .all(|c| matches!(c, 'D' | 'A' | 'H' | 'S' | 'R' | 'N' | 'V'))
        {
            continue;
        }
        if !attrs.contains('D') {
            continue;
        }
        let name_padded = match head.strip_prefix("  ") {
            Some(rest) => rest,
            None => continue,
        };
        // %-30s right-pads the name with spaces to fill its field; SMB names
        // effectively cannot have trailing spaces (Windows strips them), so
        // dropping trailing ASCII spaces recovers the real name without
        // collapsing any internal whitespace.
        let filename = name_padded.trim_end_matches(' ');
        if filename.is_empty() || filename == "." || filename == ".." {
            continue;
        }
        directories.push(filename.to_string());
    }
    directories.sort_unstable();
    directories.dedup();

    let parent_path = cleaned_path
        .rsplit_once('/')
        .map(|(parent, _)| parent.to_string())
        .or_else(|| {
            if cleaned_path.is_empty() {
                None
            } else {
                Some(String::new())
            }
        });

    (
        StatusCode::OK,
        Json(json!(ShareDirectoryBrowseResponse {
            current_path: cleaned_path,
            parent_path,
            directories,
        })),
    )
}