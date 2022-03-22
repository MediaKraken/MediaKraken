use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/playback/comic")]
pub fn user_playback_comic() -> Template {
    Template::render("bss_user/playback/bss_user_playback_comic", context! {})
}

/*
@blueprint_user_playback_comic.route('/user_comic_view/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_playback_comic.html')
@common_global.auth.login_required
async def url_bp_user_playback_comic(request, guid):
    """
    Display image comic view
    """
    db_connection = await request.app.db_pool.acquire()
    comic_data = await request.app.db_functions.db_media_path_by_uuid(guid, db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        comic_data: comic_data,
    }

 */