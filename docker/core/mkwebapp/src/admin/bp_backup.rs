use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use paginator::{Paginator, PageItem};
use core::fmt::Write;
use chrono::prelude::*;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupList {
	mm_backup_name: String,
	mm_backup_description: String,
	mm_backup_start_time: DateTime<Utc>,
	mm_backup_end_time: DateTime<Utc>,
    mm_backup_json: serde_json::Value,
}

#[derive(Serialize)]
struct TemplateBackupContext<> {
    template_data: Vec<BackupList>
}

#[get("/admin_backup")]
pub async fn admin_backup(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    Template::render("bss_admin/bss_admin_backup", {})
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