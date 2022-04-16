use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};

#[get("/admin_hardware")]
pub async fn admin_hardware(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    Template::render("bss_admin/bss_admin_hardware", {})
}

/*
@blueprint_admin_hardware.route("/admin_hardware", methods=["GET", "POST"])
@common_global.jinja_template.template('bss_admin/bss_admin_hardware.html')
@common_global.auth.login_required
async def url_bp_admin_hardware(request):
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
async def url_bp_admin_hardware_chromecast_delete(request):
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
async def url_bp_admin_hardware_chromecast_edit(request):
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
async def url_bp_admin_hardware_tvtuner_edit(request):
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
async def url_bp_admin_hardware_tvtuner_delete(request):
    """
    Delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_device_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})

 */