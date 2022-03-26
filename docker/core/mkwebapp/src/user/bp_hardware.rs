use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/hardware")]
pub async fn user_hardware() -> Template {
    Template::render("bss_user/hardware/bss_user_hardware", context! {})
}

#[get("/hardware_phue")]
pub async fn user_hardware_phue() -> Template {
    Template::render("bss_user/hardware/bss_user_hardware_phue", context! {})
}

/*
@blueprint_user_hardware.route('/user_hardware', methods=['GET'])
@common_global.jinja_template.template('bss_user/hardware/bss_user_hardware.html')
@common_global.auth.login_required
async def url_bp_user_hardware(request):
    """
    Display hardware page
    """
    db_connection = await request.app.db_pool.acquire()
    phue_hardware = await request.app.db_functions.db_hardware_device_count(
        hardware_manufacturer='Phue', db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'phue': phue_hardware
    }
 */