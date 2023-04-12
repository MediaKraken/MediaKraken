#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

use crate::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_twitch.html")]
struct UserInternetTwitchTVTemplate<'a> {
    template_data: &'a Vec<mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
}

pub async fn user_inter_twitchtv(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserInternetTwitchTVTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_internet_twitch.route('/user_internet/twitch')
@common_global.jinja_template.template('bss_user/internet/bss_user_internet_twitch.html')
@common_global.auth.login_required
pub async fn url_bp_user_internet_twitch(request):
    """
    Display twitchtv page
    """
    twitch_media = []
    for stream_data in g.twitch_api.com_net_twitch_get_featured():
        pass

    # twitch_api = common_network_twitch.CommonNetworkTwitch()
    # twitch_media = []
    # for stream_data in twitch_api.com_twitch_get_featured_streams()['featured']:
    #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
    #       message_text= {"stream": stream_data})
    #     try:
    #         if stream_data['stream']['game'] is None:
    #             twitch_media.append((stream_data['stream']['name'],
    #                                  stream_data['stream']['preview']['medium'],
    #                                  'Not Available'))
    #         else:
    #             twitch_media.append((stream_data['stream']['name'],
    #                                  stream_data['stream']['preview']['medium'],
    #                                  stream_data['stream']['game']))
    #     except:
    #         if stream_data['stream']['channel']['game'] is None:
    #             twitch_media.append((stream_data['stream']['channel']['name'],
    #                                  stream_data['stream']['preview']['medium'],
    #                                  'Not Available'))
    #         else:
    #             twitch_media.append((stream_data['stream']['channel']['name'],
    #                                  stream_data['stream']['preview']['medium'],
    #                                  stream_data['stream']['channel']['game']))
    return {
        'media': twitch_media
    }


@blueprint_user_internet_twitch.route('/user_internet/twitch_stream_detail/<stream_name>')
@common_global.jinja_template.template(
    'bss_user/internet/bss_user_internet_twitch_stream_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_internet_twitch_stream_detail(request, stream_name):
    """
    Show twitch stream detail page
    """
    # twitch_api = common_network_Twitch.com_Twitch_API()
    # media = twitch_api.com_Twitch_Channel_by_Name(stream_name)
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'twitch stream_name':
                                                                             stream_name})
    return {
        'media': stream_name
    }

 */
