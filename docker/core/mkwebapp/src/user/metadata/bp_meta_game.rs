use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;

#[derive(Serialize)]
struct TemplateMetaGameContext<> {
    template_data: Vec<mk_lib_database_metadata_game::DBMetaGameList>,
    pagination_bar: String,
}

#[get("/metadata/game/<page>")]
pub async fn user_metadata_game(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i64 = mk_lib_database_metadata_game::mk_lib_database_metadata_game_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let game_list = mk_lib_database_metadata_game::mk_lib_database_metadata_game_read(&sqlx_pool, String::new(), 0, 30).await.unwrap();
    Template::render("bss_user/metadata/bss_user_metadata_game", &TemplateMetaGameContext {
        template_data: game_list,
        pagination_bar: pagination_html,
    })
}

#[derive(Serialize)]
struct TemplateMetaGameDetailContext<> {
    template_data: serde_json::Value,
}

#[get("/metadata/game_detail/<guid>")]
pub async fn user_metadata_game_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: User, guid: rocket::serde::uuid::Uuid) -> Template {
        let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
        Template::render("bss_user/metadata/bss_user_metadata_game_detail", tera::Context::new().into_json())
}

/*
@blueprint_user_metadata_game.route('/user_meta_game', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game(request):
    """
    Display game list metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_game_software_info',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_list(offset,
                                                                  int(request.ctx.session[
                                                                          'per_page']),
                                                                  request.ctx.session[
                                                                      'search_text'],
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_game': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_game.route('/user_meta_game_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_detail(request, guid):
    """
    Display game metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await \
        request.app.db_functions.db_meta_game_by_guid(guid, db_connection=db_connection)[
            'gi_game_info_json']
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
        'data_review': None,
    }

 */