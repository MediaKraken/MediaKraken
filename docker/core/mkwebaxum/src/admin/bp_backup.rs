use crate::axum_custom_filters::filters;
use crate::guard;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_common::mk_lib_common_enum_backup_type;
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_backup.html")]
struct TemplateBackupContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_backup::DBBackupList>,
    template_backup_class: &'a Vec<(i32, String)>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_backup(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    //let current_user = auth.current_user.clone().unwrap_or_default();
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database::mk_lib_database_backup::mk_lib_database_backup_count(&sqlx_pool)
            .await
            .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/backup".to_string(),
    )
    .await
    .unwrap();
    let backup_list = mk_lib_database::mk_lib_database_backup::mk_lib_database_backup_read(
        &sqlx_pool, db_offset, 30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if backup_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateBackupContext {
        template_data: &backup_list,
        template_backup_class: &mk_lib_common_enum_backup_type::BACKUP_CLASS.clone(),
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
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
        // do the actual delete
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
