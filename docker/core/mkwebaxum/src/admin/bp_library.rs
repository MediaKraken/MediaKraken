use crate::AppState;
use crate::mk_lib_database;
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
use serde_json::{Value, json};
use sqlx::postgres::PgPool;
use tokio::process::Command;
use tracing::{error, info};

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

fn classify_smbclient_browse_error(
    stdout_output: &str,
    stderr_output: &str,
) -> (StatusCode, &'static str) {
    let combined_output = format!("{stdout_output}\n{stderr_output}").to_ascii_lowercase();

    if combined_output.contains("nt_status_access_denied")
        || combined_output.contains("access denied")
        || combined_output.contains("permission denied")
    {
        return (
            StatusCode::FORBIDDEN,
            "Share is reachable but access was denied",
        );
    }

    if combined_output.contains("nt_status_object_path_not_found")
        || combined_output.contains("nt_status_object_name_not_found")
        || combined_output.contains("nt_status_bad_network_name")
        || combined_output.contains("no such file")
        || combined_output.contains("cannot chdir")
    {
        return (StatusCode::NOT_FOUND, "Share path was not found");
    }

    if combined_output.contains("nt_status_bad_network_path")
        || combined_output.contains("nt_status_network_name_deleted")
        || combined_output.contains("connection to")
        || combined_output.contains("connection refused")
        || combined_output.contains("could not resolve")
        || combined_output.contains("host is down")
        || combined_output.contains("name or service not known")
        || combined_output.contains("timed out")
    {
        return (StatusCode::BAD_GATEWAY, "Unable to reach share");
    }

    (StatusCode::BAD_GATEWAY, "Failed to list share directories")
}

pub async fn admin_library(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    tracing::info!("Admin library request received");
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
        let share_list =
            mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_read(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let library_list =
            mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_audit_read(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let share_user_list =
        mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_user_read(
           &state.sqlx_pool_ro,
        )
        .await
        .unwrap();
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
        let reply_html = template.render().unwrap();
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
                .unwrap();
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mkmediascanner",
            json!({"Type": "Library Scan"}).to_string(),
        )
        .await
        .unwrap();
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            .unwrap();
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
                .unwrap();
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mksharescanner",
            json!({"Type": "Share Scan", "Data": "192.168.1"}).to_string(),
        )
        .await
        .unwrap();
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            .unwrap();
        Redirect::to("/admin/library")
    }
}

pub async fn admin_library_share_directories(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<ShareDirectoryBrowseQuery>,
) -> impl IntoResponse {
    tracing::info!("Browsing share directories request received");
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
    tracing::info!("Share directory request authorized");
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
    tracing::info!(share_guid = %query.share_guid, "Loaded share details for directory browse");
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

    let share_uri = format!(
        "//{}/{}",
        share_info.mm_network_share_ip,
        share_info
            .mm_network_share_path
            .trim_matches('/')
            .rsplit_once('\\')
            .unwrap()
            .1,
    );

    let smb_commands: Vec<String> = vec![
        String::from("recurse OFF"),
        String::from("prompt OFF"),
        String::from("ls"),
    ];

    let mut smb_command = Command::new("smbclient");
    smb_command
        .arg(share_uri)
        .arg("-g")
        .arg("-c")
        .arg(smb_commands.join(";"));
    if cleaned_path.is_empty() == false {
        smb_command.arg("-D").arg(&cleaned_path);
    }
    if let Some(workgroup) = share_info.mm_network_share_workgroup.as_deref() {
        if workgroup.is_empty() == false {
            smb_command.arg("-W").arg(workgroup);
        }
    }
    if let Some(user) = share_info.mm_share_auth_user.as_deref()
        && user != "guest"
    {
        let pass = share_info
            .mm_share_auth_password
            .as_deref()
            .unwrap_or_default();
        smb_command.arg("-U").arg(format!("{}%{}", user, pass));
    } else {
        smb_command.arg("-N");
    }
    tracing::info!(?smb_command, "Running smbclient directory listing command");
    let smb_output = match smb_command.output().await {
        Ok(data) => data,
        Err(error) => {
            tracing::error!(?error, "smbclient execution failed");
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
        let (status_code, error_message) =
            classify_smbclient_browse_error(&stdout_output, &stderr_output);
        tracing::error!(
            status_code = ?smb_output.status.code(),
            stdout = %stdout_output,
            stderr = %stderr_output,
            "smbclient failed"
        );
        return (
            status_code,
            Json(json!({"error": error_message, "details": details_output})),
        );
    }

    let stdout_data = String::from_utf8_lossy(&smb_output.stdout);
    let mut directories = Vec::new();
    for line in stdout_data.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 || parts[0] != "D" {
            continue;
        }
        if parts[1] == "." || parts[1] == ".." {
            continue;
        }
        directories.push(parts[1].to_string());
    }
    directories.sort_unstable();

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

/*

@blueprint_admin_library.route("/admin_library_edit", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_library_edit.html')
@common_global.auth.login_required
pub async fn url_bp_admin_library_edit(request):
    """
    allow user to edit lib
    """
    form = BSSLibraryAddEditForm(request)
    db_connection = await request.app.db_pool.acquire()
    if request.method == 'POST':
        if form.validate_on_submit():
            if request.form['action_type'] == 'Add':
                # check for UNC
                if request.form['library_path'][:1] == "\\":
                    addr, share, path = common_string.com_string_unc_to_addr_path(
                        request.form['library_path'])
                    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                        message_type='info',
                        message_text=
                        {'smb info': addr,
                         'share': share,
                         'path': path})
                    if addr is None:  # total junk path for UNC
                        request['flash']('Invalid UNC path.', 'error')
                        return redirect(
                            request.app.url_for(
                                'name_blueprint_admin_library.url_bp_admin_library_edit'))
                    smb_stuff = common_network_cifs.CommonCIFSShare()
                    smb_stuff.com_cifs_connect(addr)
                    if not smb_stuff.com_cifs_share_directory_check(share, path):
                        smb_stuff.com_cifs_close()
                        request['flash']("Invalid UNC path.", 'error')
                        return redirect(
                            request.app.url_for(
                                'name_blueprint_admin_library.url_bp_admin_library_edit'))
                    smb_stuff.com_cifs_close()
                // TODO these should be mounted under mkmount on docker host
                # which will break docker swarm....when master moves
                # # smb/cifs mounts
                # else if request.form['library_path'][0:3] == "smb":
                #     smb_stuff = common_network_cifs.CommonCIFSShare()
                #     smb_stuff.com_cifs_connect(
                #         ip_addr, user_name='guest', user_password='')
                #     smb_stuff.com_cifs_share_directory_check(
                #         share_name, dir_path)
                #     smb_stuff.com_cifs_close()
                # # nfs mount
                # else if request.form['library_path'][0:3] == "nfs":
                #     pass
                else if not os.path.isdir(os.path.join('/mediakraken/mnt',
                                                    request.form['library_path'])):
                    request['flash']("Invalid library path.", 'error')
                    return redirect(
                        request.app.url_for(
                            'name_blueprint_admin_library.url_bp_admin_library_edit'))
                # verify it doesn't exist and add
                if await request.app.db_functions.db_library_path_check(request.form[
                                                                            'library_path'],
                                                                        db_connection) == 0:
                    await request.app.db_functions.db_library_path_add(request.form[
                                                                           'library_path'],
                                                                       request.form[
                                                                           'Lib_Class'],
                                                                       None, db_connection)
                    return redirect(
                        request.app.url_for('name_blueprint_admin_library.url_bp_admin_library'))
                else:
                    request['flash']("Path already in library.", 'error')
                    return redirect(
                        request.app.url_for(
                            'name_blueprint_admin_library.url_bp_admin_library_edit'))
            else if request.form['action_type'] == 'Browse...':  # popup browse form
                pass
            # popup browse form for synology
            else if request.form['action_type'] == 'Synology':
                pass
        else:
            flash_errors(form)
    await request.app.db_pool.release(db_connection)
    return {
        'form': form,
        'data_class': ((common_global.DLMediaType.Movie.name,
                        common_global.DLMediaType.Movie.value),
                       (common_global.DLMediaType.TV.name,
                        common_global.DLMediaType.TV.value),
                       (common_global.DLMediaType.Music.name,
                        common_global.DLMediaType.Music.value),
                       (common_global.DLMediaType.Sports.name,
                        common_global.DLMediaType.Sports.value),
                       (common_global.DLMediaType.Game.name,
                        common_global.DLMediaType.Game.value),
                       (common_global.DLMediaType.Publication.name,
                        common_global.DLMediaType.Publication.value),
                       (common_global.DLMediaType.Picture.name,
                        common_global.DLMediaType.Picture.value),
                       (common_global.DLMediaType.Anime.name,
                        common_global.DLMediaType.Anime.value),
                       (common_global.DLMediaType.Adult.name,
                        common_global.DLMediaType.Adult.value)),
    }


@blueprint_admin_library.route('/admin_library_update', methods=['POST'])
@common_global.auth.login_required
pub async fn url_bp_admin_library_update(request):
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_library_path_update_by_uuid(request.form['new_path'],
                                                                  request.form['new_class'],
                                                                  request.form['id'],
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})

 */
