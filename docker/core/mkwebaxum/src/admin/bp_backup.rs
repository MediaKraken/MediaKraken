#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use bytesize::ByteSize;
use chrono::prelude::*;
use core::fmt::Write;
use paginator::{PageItem, Paginator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

mod filters {
    pub fn space_to_html(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(" ", "%20"))
    }

    pub fn slash_to_asterik(s: &str) -> ::askama::Result<String> {
        Ok(s.replace("/", "*"))
    }
}

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

use crate::mk_lib_database_user;

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupList {
    mm_backup_name: String,
    mm_backup_description: String,
    mm_backup_start_time: DateTime<Utc>,
    mm_backup_end_time: DateTime<Utc>,
    mm_backup_json: serde_json::Value,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_backup.html")]
struct TemplateBackupContext<'a> {
    template_data: &'a Vec<BackupList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_backup(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = TemplateBackupContext {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_admin_backup.route("/admin_backup", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_backup.html.tera')
@common_global.auth.login_required
pub async fn url_bp_admin_backup(request):
    form = BSSBackupEditForm(request)
    errors = {}
    if request.method == 'POST':
        if form.validate_on_submit():
            print('validated', flush=True)
            print(request.form['backup'], flush=True)
            if request.form['backup'] == ['Update Backup']:
                pass
            else if request.form['backup'] == ['Submit Backup']:
                print('startbackup', flush=True)
                common_database.com_database_backup()
                request['flash']("Postgresql Database Backup Task Submitted.", "success")
        else:
            flash_errors(form)
    backup_enabled = False
    backup_files = []
    local_file_backups = common_file.com_file_dir_list(dir_name='/mediakraken/backup',
                                                       filter_text='dump', walk_dir=False,
                                                       skip_junk=False, file_size=True,
                                                       directory_only=False, file_modified=False)
    if local_file_backups != None:
        for backup_local in local_file_backups:
            backup_files.append((backup_local[0], 'Local',
                                 common_string.com_string_bytes2human(backup_local[1])))
    // TODO
    # # cloud backup list
    # if len(g.option_config_json['Cloud']) > 0:  # to see if the json has been populated
    #     cloud_handle = common_network_cloud.CommonLibCloud(g.option_config_json)
    #     for backup_cloud in cloud_handle.com_net_cloud_list_data_in_container(
    #             g.option_config_json['MediaKrakenServer']['BackupContainerName']):
    #         backup_files.append((backup_cloud.name, backup_cloud.type,
    #                              common_string.com_string_bytes2human(backup_cloud.size)))
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/admin/admin_backup',
                                                                      item_count=len(backup_files),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    return {
        'form': form,
        'errors': errors,
        'backup_list': sorted(backup_files, reverse=True),
        'data_interval': ('Hours', 'Days', 'Weekly'),
        'data_class': common_network_cloud.com_libcloud_storage_provider_list(),
        'data_enabled': backup_enabled,
        'pagination_bar': pagination,
        'page': page,
        'per_page': int(request.ctx.session['per_page'])
    }


@blueprint_admin_backup.route('/admin_backup_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_backup_delete(request):
    """
    Delete backup file action 'page'
    """
    file_path, file_type = request.form['id'].split('|')
    if file_type == "Local":
        os.remove(file_path)
    else:
        pass
        // TODO, do the actual delete
    return json.dumps({'status': 'OK'})


@blueprint_admin_backup.route('/admin_backup_restore/<backup_filename>', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_backup_restore(request, backup_filename):
    """
    run restore script on db container
    """
    # since I have to strip the first / as url_for gets mad
    common_database.com_database_restore(backup_filename.replace('*', '/'))
    return json.dumps({'status': 'OK'})

 */
