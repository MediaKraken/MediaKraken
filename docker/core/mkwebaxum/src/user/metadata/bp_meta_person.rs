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
use sqlx::postgres::PgPool;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_person.rs"]
mod mk_lib_database_metadata_person;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person.html")]
struct TemplateMetaPersonContext {
    template_data: Vec<mk_lib_database_metadata_person::DBMetaPersonList>,
    pagination_bar: String,
}

#[get("/metadata/person/<page>")]
pub async fn user_metadata_person(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_person::mk_lib_database_metadata_person_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
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
    Template::render(
        "bss_user/metadata/bss_user_metadata_person.html",
        &TemplateMetaPersonContext {
            template_data: person_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMetaPersonDetailContext {
    template_data: serde_json::Value,
}

#[get("/metadata/person_detail/<guid>")]
pub async fn user_metadata_person_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_person_detail.html",
        tera::Context::new().into_json(),
    )
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
