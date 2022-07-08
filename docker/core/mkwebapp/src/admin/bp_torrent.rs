use core::fmt::Write;
use paginator::{PageItem, Paginator};
use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[get("/admin_torrent")]
pub async fn admin_torrent(user: AdminUser) -> Template {
    Template::render(
        "bss_admin/bss_admin_torrent",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_admin_torrent.route("/admin_torrent")
@common_global.jinja_template.template('bss_admin/bss_admin_torrent.html')
@common_global.auth.login_required
pub async fn url_bp_admin_torrent(request):
    """
    Display torrent page
    """
    db_connection = await request.app.db_pool.acquire()
    trans_connection = common_network_torrent.CommonTransmission(
        await request.app.db_functions.db_opt_json_read(db_connection=db_connection))
    await request.app.db_pool.release(db_connection)
    transmission_data = []
    if trans_connection != None:
        torrent_no = 1
        for torrent in trans_connection.com_trans_get_torrent_list():
            transmission_data.append(
                (common_internationalization.com_inter_number_format(torrent_no),
                 torrent.name, torrent.hashString, torrent.status,
                 torrent.progress, torrent.ratio))
            torrent_no += 1
    return {
        'data_torrent': transmission_data
    }


@blueprint_admin_torrent.route('/admin_torrent_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_torrent_delete(request):
    """
    Delete torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_torrent.route('/admin_torrent_edit', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_torrent_edit(request):
    """
    Edit a torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})

 */
