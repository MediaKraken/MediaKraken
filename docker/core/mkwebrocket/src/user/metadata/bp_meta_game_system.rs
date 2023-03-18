#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use sqlx::postgres::PgRow;
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_game_system.rs"]
mod mk_lib_database_metadata_game_system;

#[derive(Serialize)]
struct TemplateMetaGameSystemContext {
    template_data: Vec<mk_lib_database_metadata_game_system::DBMetaGameSystemList>,
    pagination_bar: String,
}

#[get("/metadata/game_system/<page>")]
pub async fn user_metadata_game_system(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_count(
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
        "/user/metadata/game_system".to_string(),
    )
    .await
    .unwrap();
    let game_system_list =
        mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_game_system",
        &TemplateMetaGameSystemContext {
            template_data: game_system_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMetaGameSystemDetailContext {
    template_data: serde_json::Value,
}

#[get("/metadata/game_system_detail/<guid>")]
pub async fn user_metadata_game_system_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    let detail_data =
        mk_lib_database_metadata_game_system::mk_lib_database_metadata_game_system_detail(
            &sqlx_pool, tmp_uuid,
        )
        .await
        .unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_game_system_detail",
        &TemplateMetaGameSystemDetailContext {
            template_data: detail_data,
        },
    )
}

/*
@blueprint_user_metadata_game_system.route('/user_meta_game_system', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_system.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_system(request):
    """
    Display list of game system metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game_system'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_meta_game_system_list_count(
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_system_list(offset,
                                                                         int(request.ctx.session[
                                                                                 'per_page']),
                                                                         request.ctx.session[
                                                                             'search_text'],
                                                                         db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_game_system.route('/user_meta_game_system_detail/<guid>')
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_game_system_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_system_detail(request, guid):
    """
    Display metadata game detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_game_system_by_guid(guid,
                                                                            db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
    }

 */
