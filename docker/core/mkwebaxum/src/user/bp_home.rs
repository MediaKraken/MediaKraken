#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_user/bss_user_home.html")]
struct TemplateUserHomeContext {
    template_data_new_media: bool,
    template_data_user_media_queue: bool,
}

#[get("/home")]
pub async fn user_home(user: User) -> Template {
    Template::render(
        "bss_user/bss_user_home.html",
        &TemplateUserHomeContext {
            template_data_new_media: false,
            template_data_user_media_queue: false,
        },
    )
}

/*
@blueprint_user_homepage.route('/user_home', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_home.html')
@common_global.auth.login_required
pub async fn url_bp_user_homepage(request):
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