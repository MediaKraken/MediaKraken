use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../mk_lib_database_cron.rs"]
mod mk_lib_database_cron;

#[get("/admin_cron")]
pub async fn admin_cron(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    let cron_list = mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool).await.unwrap();
    Template::render("bss_admin/bss_admin_cron", context! {
        media_cron: cron_list,
    })
}

/*

@blueprint_admin_cron.route('/admin_cron_delete', methods=["POST"])
@common_global.auth.login_required
async def url_bp_admin_cron_delete(request):
    """
    Delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_cron_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_cron.route('/admin_cron_edit/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_admin/bss_admin_cron_edit.html')
@common_global.auth.login_required
async def url_bp_admin_cron_edit(request, guid):
    """
    Edit cron job page
    """
    form = BSSCronEditForm(request, csrf_enabled=False)
    if request.method == 'POST':
        if form.validate_on_submit():
            request.form['name']
            request.form['description']
            request.form['enabled']
            request.form['interval']
            request.form['time']
            request.form['json']
    return {
        'guid': guid, 'form': form
    }


@blueprint_admin_cron.route('/admin_cron_run/<guid>', methods=['GET', 'POST'])
@common_global.auth.login_required(user_keyword='user')
async def url_bp_admin_cron_run(request, user, guid):
    """
    Run cron jobs
    """
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'admin cron run': guid})
    db_connection = await request.app.db_pool.acquire()
    cron_job_data = await request.app.db_functions.db_cron_info(guid, db_connection)
    cron_json_data = cron_job_data['mm_cron_json']
    # submit the message
    common_network_pika.com_net_pika_send({'Type': cron_json_data['Type'],
                                           'User': user.id,
                                           'JSON': cron_json_data},
                                          exchange_name=cron_json_data[
                                              'exchange_key'],
                                          route_key=cron_json_data['route_key'])
    await request.app.db_functions.db_cron_time_update(cron_job_data['mm_cron_name'],
                                                       db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return redirect(request.app.url_for('name_blueprint_admin_cron.url_bp_admin_cron'))

 */