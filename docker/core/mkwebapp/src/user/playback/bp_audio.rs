use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/playback/audio")]
pub async fn user_playback_audio(user: User) -> Template {
    Template::render("bss_user/playback/bss_user_album_playback", tera::Context::new().into_json())
}

/*
@blueprint_user_playback_audio.route('/user_play_album/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_album_playback.html')
@common_global.auth.login_required
pub async fn url_bp_user_playback_album(request, guid):
    """
    Obsolete?
    """
    db_connection = await request.app.db_pool.acquire()
    data_desc = await request.app.db_functions.db_meta_music_album_by_guid(guid, db_connection=db_connection)
    data_song_list = await request.app.db_functions.db_meta_music_songs_by_album_guid(guid,
                                                                                      db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        data_desc: data_desc,
        data_song_list: data_song_list,
    }

 */