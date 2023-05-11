use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Serialize)]
struct TemplateUserHardwareContext {
    template_data_phue: i32,
}

#[get("/hardware")]
pub async fn user_hardware(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User) -> Template {
    Template::render(
        "bss_user/hardware/bss_user_hardware",
        &TemplateUserHardwareContext {
            template_data_phue: 0,
        },
    )
}

#[get("/hardware_phue")]
pub async fn user_hardware_phue(user: User) -> Template {
    Template::render(
        "bss_user/hardware/bss_user_hardware_phue",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_user_hardware.route('/user_hardware', methods=['GET'])
@common_global.jinja_template.template('bss_user/hardware/bss_user_hardware.html')
@common_global.auth.login_required
pub async fn url_bp_user_hardware(request):
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
