use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_database;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_hardware.html")]
struct AdminHardwareTemplate<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_hardware_device::DBDeviceList>,
    template_data_exists: &'a bool,
}

pub async fn admin_hardware(
    Extension(sqlx_pool): Extension<PgPool>,
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
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let hardware_list =
            mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_device_read(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let mut hardware_data: bool = false;
        if hardware_list.len() > 0 {
            hardware_data = true;
        }
        let template = AdminHardwareTemplate {
            template_data: &hardware_list,
            template_data_exists: &hardware_data,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*
@blueprint_admin_hardware.route("/admin_hardware", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_hardware.html')
@common_global.auth.login_required
pub async fn url_bp_admin_hardware(request):
    if request.method == 'POST':
        # submit the message
        common_network_pika.com_net_pika_send({'Type': 'Hardware Scan'},
                                              rabbit_host_name='mkstack_rabbitmq',
                                              exchange_name='mkque_hardware_ex',
                                              route_key='mkhardware')
        request['flash']("Scheduled hardware scan.", "success")
    chromecast_list = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_device_list('Chromecast',
                                                                  db_connection=db_connection):
        if row_data['mm_device_json']['Model'] == 'Eureka Dongle':
            device_model = 'Chromecast'
        else:
            device_model = row_data['mm_device_json']['Model']
        chromecast_list.append((row_data['mm_device_id'],
                                row_data['mm_device_json']['Name'],
                                device_model,
                                row_data['mm_device_json']['IP'],
                                True))
    tv_tuners = []
    for row_data in await request.app.db_functions.db_device_list('tvtuner',
                                                                  db_connection=db_connection):
        tv_tuners.append((row_data['mm_device_id'], row_data['mm_device_json']['HWModel']
                          + " (" + row_data['mm_device_json']['Model'] + ")",
                          row_data['mm_device_json']['IP'],
                          len(row_data['mm_device_json']['Channels']),
                          row_data['mm_device_json']['Firmware'],
                          row_data['mm_device_json']['Active']))
    await request.app.db_pool.release(db_connection)
    return {
        'data_chromecast': chromecast_list,
        'data_tuners': tv_tuners,
    }


@blueprint_admin_hardware.route('/admin_hardware_chromecast_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_hardware_chromecast_delete(request):
    """
    Delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_device_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_hardware.route("/admin_hardware_chromecast_edit", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_hardware_chromecast_edit.html')
@common_global.auth.login_required
pub async fn url_bp_admin_hardware_chromecast_edit(request):
    """
    allow user to edit chromecast
    """
    form = BSSChromecastEditForm(request)
    if request.method == 'POST':
        if form.validate_on_submit():
            if request.form['action_type'] == 'Add':
                # verify it doesn't exit and add
                db_connection = await request.app.db_pool.acquire()
                if await request.app.db_functions.db_device_check(request.form['name'],
                                                                  request.form['ipaddr'],
                                                                  db_connection=db_connection) == 0:
                    request.app.db_functions.db_device_upsert('cast',
                                                              json.dumps(
                                                                  {'Name': request.form['name'],
                                                                   'Model': "NA",
                                                                   'IP': request.form['ipaddr']}),
                                                              db_connection=db_connection)
                    await request.app.db_pool.release(db_connection)
                    return redirect(request.app.url_for('admins_chromecasts.admin_chromecast'))
                else:
                    request['flash']("Chromecast already in database.", 'error')
                    await request.app.db_pool.release(db_connection)
                    return redirect(
                        request.app.url_for('admins_chromecasts.admin_chromecast_edit_page'))
        else:
            flash_errors(form)
    return {
        'form': form
    }


@blueprint_admin_hardware.route("/admin_hardware_tvtuner_edit", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_hardware_tuner_edit.html')
@common_global.auth.login_required
pub async fn url_bp_admin_hardware_tvtuner_edit(request):
    """
    allow user to edit tuner
    """
    form = BSSTVTunerEditForm(request)
    if request.method == 'POST':
        if form.validate_on_submit():
            if request.form['action_type'] == 'Add':
                # verify it doesn't exit and add
                db_connection = await request.app.db_pool.acquire()
                if await request.app.db_functions.db_device_check(request.form['name'],
                                                                  request.form['ipaddr'],
                                                                  db_connection) == 0:
                    await request.app.db_functions.db_device_upsert('tvtuner',
                                                                    json.dumps(
                                                                        {'Name': request.form[
                                                                            'name'],
                                                                         'Model': "NA",
                                                                         'IP': request.form[
                                                                             'ipaddr']}),
                                                                    db_connection)
                    await request.app.db_pool.release(db_connection)
                    return redirect(request.app.url_for('admins_tvtuners.admin_tvtuners'))
                else:
                    request['flash']("TV Tuner already in database.", 'error')
                    await request.app.db_pool.release(db_connection)
                    return redirect(
                        request.app.url_for('admins_tvtuners.admin_tuner_edit_page'))
        else:
            flash_errors(form)
    return {
        'form': form
    }


@blueprint_admin_hardware.route('/admin_hardware_tvtuner_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_hardware_tvtuner_delete(request):
    """
    Delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_device_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})

 */
