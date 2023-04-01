#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_youtube.html")]
struct UserInternetYoutubeTemplate<'a> {
    template_data: &'a Vec<mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
}

pub async fn user_inter_youtube() -> impl IntoResponse {
    let template = UserInternetYoutubeTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_youtube_detail.html")]
struct UserInternetYoutubeDetailTemplate;

pub async fn user_inter_youtube_detail() -> impl IntoResponse {
    let template = UserInternetYoutubeDetailTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_internet_youtube.route('/user_internet/youtube', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/internet/bss_user_internet_youtube.html')
@common_global.auth.login_required
pub async fn url_bp_user_internet_youtube(request):
    """
    Display youtube page
    """
    youtube_videos = []
    if request.ctx.session['search_text'] != None:
        // TODO - ytpy search instead
        videos, channels, playlists = g.google_instance.com_google_youtube_search(
            request.ctx.session['search_text'])
        for url_link in videos:
            await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                             message_text={
                                                                                 'searchurllink': url_link})
            youtube_videos.append(
                g.google_instance.com_google_youtube_info(url_link, 'snippet'))
    else:
        # get trending for specified country code
        for url_link in common_network_youtube.com_net_yt_trending(locale.getdefaultlocale()[0]):
            await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                             message_text={
                                                                                 'urllink': url_link})
            youtube_videos.append(g.google_instance.com_google_youtube_info(url_link, 'snippet'))
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'temphold': youtube_videos})
    return {
        'media': youtube_videos
    }


@blueprint_user_internet_youtube.route('/user_internet/youtube_detail/<uuid>')
@common_global.jinja_template.template('bss_user/internet/bss_user_internet_youtube_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_internet_youtube_detail(request, uuid):
    """
    Display youtube details page
    """
    return {
        'media': g.google_instance.com_google_youtube_info(uuid),
        'data_guid': uuid
    }

 */
