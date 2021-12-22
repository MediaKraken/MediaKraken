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