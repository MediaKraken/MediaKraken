#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_library.rs"]
mod mk_lib_database_library;

#[derive(Serialize)]
struct TemplateAdminLibraryContext {
    template_data: Vec<mk_lib_database_library::DBLibraryList>,
    pagination_bar: String,
}

#[get("/library/<page>")]
pub async fn admin_library(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
    page: i32,
) -> Template {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database_library::mk_lib_database_library_count(&sqlx_pool)
        .await
        .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/admin_library".to_string(),
    )
    .await
    .unwrap();
    let library_list =
        mk_lib_database_library::mk_lib_database_library_read(&sqlx_pool, db_offset, 30)
            .await
            .unwrap();
    Template::render(
        "bss_admin/bss_admin_library",
        &TemplateAdminLibraryContext {
            template_data: library_list,
            pagination_bar: pagination_html,
        },
    )
}

/*
    if request.method == 'POST':
        if "scan" in request.form:
            # submit the message
            common_network_pika.com_net_pika_send({'Type': 'Library Scan'},
                                                  rabbit_host_name='mkstack_rabbitmq',
                                                  exchange_name='mkque_ex',
                                                  route_key='mkque')
            // TODO request['flash']('Scheduled media scan.', 'success')

@blueprint_admin_library.route('/admin_library_by_id', methods=['POST'])
@common_global.auth.login_required
pub async fn url_bp_admin_library_by_id(request):
    db_connection = await request.app.db_pool.acquire()
    result = await request.app.db_functions.db_library_path_by_uuid(request.form['id'],
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'Id': result['mm_media_dir_guid'],
                       'Path': result['mm_media_dir_path'],
                       'Media Class': result['mm_media_dir_class_type']})

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
                #     // TODO
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
