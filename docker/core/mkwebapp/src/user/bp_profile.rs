use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[path = "../mk_lib_database_user_profile.rs"]
mod mk_lib_database_user_profile;

#[get("/profile")]
pub async fn user_profile(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    Template::render("bss_user/bss_user_profile", context! {})
}

/*
@blueprint_user_profile.route('/user_profile', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_profile.html')
@common_global.auth.login_required
async def url_bp_user_profile(request):
    return {}

 */