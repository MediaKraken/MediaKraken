#![cfg_attr(debug_assertions, allow(dead_code))]

use stdext::function_name;
use serde_json::json;
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
use sqlx::postgres::PgPool;

use crate::mk_lib_logging;

use crate::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_hardware.html")]
struct AdminHardwareTemplate<'a> {
    template_data_alexa: &'a serde_json::Value,
    template_data_chromecast: &'a serde_json::Value,
    template_data_crestron: &'a serde_json::Value,
    template_data_dlna: &'a serde_json::Value,
    template_data_hdhomerun: &'a serde_json::Value,
    template_data_phue: &'a serde_json::Value,
    template_data_roku: &'a serde_json::Value,
    template_data_soco: &'a serde_json::Value,
    page: &'a usize,
};

pub async fn admin_hardware(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> impl IntoResponse {
    let template = AdminHardwareTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
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
