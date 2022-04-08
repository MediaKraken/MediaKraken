use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
//use rocket_auth::{Users, Auth, Signup, Login, User};
use rocket_auth::{prelude::Error, *};
use rocket::{get, post, form::Form, routes};
use rocket::{form::*, State};

#[get("/home")]
pub async fn user_home() -> Template {
    Template::render("bss_user/bss_user_home", context! {})
}

/*
@blueprint_user_homepage.route('/user_home', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_home.html')
@common_global.auth.login_required
async def url_bp_user_homepage(request):
    """
    Display user home page
    """
    print('current user - url_bp_user_homepage', common_global.auth.current_user(request),
          flush=True)
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_media_new(days_old=7,
                                                             db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'data_new_media': media_data,
        'data_user_media_queue': False
    }

 */