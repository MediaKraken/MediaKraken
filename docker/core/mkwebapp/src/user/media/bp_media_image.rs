#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_database_media_images.rs"]
mod mk_lib_database_media_images;

#[get("/media/image")]
pub async fn user_media_image(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User) -> Template {
    Template::render(
        "bss_user/media/bss_user_media_image_gallery",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_user_image.route('/user_imagegallery')
@common_global.jinja_template.template('bss_user/media/bss_user_media_image_gallery.html')
@common_global.auth.login_required
pub async fn url_bp_user_image_gallery(request):
    """
    Display image gallery page
    """
    db_connection = await request.app.db_pool.acquire()
    image_data = await request.app.db_functions.db_image_list(common_global.DLMediaType.Picture,
                                                              db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {'image_data': image_data
            }

 */
