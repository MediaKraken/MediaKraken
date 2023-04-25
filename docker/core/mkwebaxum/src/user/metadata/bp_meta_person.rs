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
use sqlx::postgres::PgPool;
use stdext::function_name;

use crate::mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

use crate::database::mk_lib_database_metadata_person;

use crate::database::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person.html")]
struct TemplateMetaPersonContext<'a> {
    template_data: &'a Vec<mk_lib_database_metadata_person::DBMetaPersonList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_person(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database_metadata_person::mk_lib_database_metadata_person_count(
        &sqlx_pool,
        String::new(),
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/person".to_string(),
    )
    .await
    .unwrap();
    let person_list = mk_lib_database_metadata_person::mk_lib_database_metadata_person_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if person_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMetaPersonContext {
        template_data: &person_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person_detail.html")]
struct TemplateMetaPersonDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_metadata_person_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMetaPersonDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*

@blueprint_user_metadata_people.route('/user_meta_person_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_person_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_person_detail(request, guid):
    """
    Display person detail page
    """
    db_connection = await request.app.db_pool.acquire()
    person_data = await request.app.db_functions.db_meta_person_by_guid(guid=guid,
                                                                        db_connection=db_connection)
    if person_data['mmp_person_image'] != None:
        try:
            person_image = person_data['mmp_person_image'] + person_data['mmp_meta']
        except:
            person_image = "img/person_missing.png"
    else:
        person_image = "img/person_missing.png"
    media_data = await request.app.db_functions.db_meta_person_as_seen_in(person_data['mmp_id'],
                                                                          db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'json_metadata': person_data['mmp_person_meta_json'],
        'data_person_image': person_image,
        'data_also_media': media_data,
    }


@blueprint_user_metadata_people.route('/user_meta_person_list', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_person.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_person_list(request):
    """
    Display person list page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    person_list = []
    db_connection = await request.app.db_pool.acquire()
    for person_data in await request.app.db_functions.db_meta_person_list(offset,
                                                                          int(request.ctx.session[
                                                                                  'per_page']),
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'person data': person_data,
                                                                             'im':
                                                                                 person_data[
                                                                                     'mmp_person_image'],
                                                                             'meta': person_data[
                                                                                 'mmp_meta']})
        if person_data['mmp_person_image'] != None:
            try:
                person_image = person_data['mmp_person_image'] + person_data['mmp_meta']
            except:
                person_image = "img/person_missing.png"
        else:
            person_image = "img/person_missing.png"
        person_list.append(
            (person_data['mmp_id'], person_data['mmp_person_name'], person_image.replace('"', '')))
    request.ctx.session['search_page'] = 'meta_people'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_person_list',
                                                                      item_count=await request.app.db_functions.db_meta_person_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media_person': person_list,
        'pagination_bar': pagination,
    }

 */
