#![cfg_attr(debug_assertions, allow(dead_code))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user_profile.rs"]
mod mk_lib_database_user_profile;

#[get("/profile")]
pub async fn user_profile(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User) -> Template {
    Template::render(
        "bss_user/bss_user_profile",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_user_profile.route('/user_profile', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_profile.html')
@common_global.auth.login_required
pub async fn url_bp_user_profile(request):
    return {}

 */
