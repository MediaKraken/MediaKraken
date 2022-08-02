#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[path = "../mk_lib_database_sync.rs"]
mod mk_lib_database_sync;

#[derive(Serialize)]
struct TemplateSyncContext {
    template_data: Vec<mk_lib_database_sync::DBSyncList>,
}

#[get("/sync")]
pub async fn user_sync(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User) -> Template {
    let sync_list =
        mk_lib_database_sync::mk_lib_database_sync_list(&sqlx_pool, uuid::Uuid::nil(), 0, 30)
            .await
            .unwrap();
    Template::render(
        "bss_user/media/bss_user_media_sync",
        &TemplateSyncContext {
            template_data: sync_list,
        },
    )
}

/*

@blueprint_user_sync.route('/user_sync')
@common_global.jinja_template.template('bss_user/media/bss_user_media_sync.html')
@common_global.auth.login_required
pub async fn url_bp_user_sync_display_all(request):
    """
    Display sync page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    # 0 - mm_sync_guid uuid, 1 - mm_sync_path, 2 - mm_sync_path_to, 3 - mm_sync_options_json
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_sync',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_sync',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_sync_list(offset,
                                                             int(request.ctx.session[
                                                                     'per_page']),
                                                             db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_sync': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_sync.route('/user_sync_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_user_admin_sync_delete_page(request):
    """
    Display sync delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_sync_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_user_sync.route('/user_sync_edit/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/user_sync_edit.html')
@common_global.auth.login_required
pub async fn url_bp_user_sync_edit(request, guid):
    """
    Allow user to edit sync page
    """
    if request.method == 'POST':
        db_connection = await request.app.db_pool.acquire()
        sync_json = {'Type': request.form['target_type'],
                     'Media GUID': guid,
                     'Options': {'VContainer': request.form['target_container'],
                                 'VCodec': request.form['target_codec'],
                                 'Size': request.form['target_file_size'],
                                 'AudioChannels': request.form['target_audio_channels'],
                                 'ACodec': request.form['target_audio_codec'],
                                 'ASRate': request.form['target_sample_rate']},
                     'Priority': request.form['target_priority'],
                     'Status': 'Scheduled',
                     'Progress': 0}
        await request.app.db_functions.db_sync_insert(request.form['name'],
                                                      request.form['target_output_path'],
                                                      sync_json,
                                                      db_connection=db_connection)
        await request.app.db_pool.release(db_connection)
        return redirect(
            request.app.url_for('name_blueprint_user_movie.url_bp_user_movie_detail', guid=guid))
    form = BSSSyncEditForm(request, csrf_enabled=False)
    if form.validate_on_submit():
        pass
    else:
        flash_errors(form)
    return {
        'guid': guid,
        'form': form
    }

 */
