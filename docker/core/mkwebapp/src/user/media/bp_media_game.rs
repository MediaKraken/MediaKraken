use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_game.rs"]
mod mk_lib_database_media_game;

#[derive(Serialize)]
struct TemplateMediaGameContext<> {
    template_data: Vec<mk_lib_database_media_game::DBMediaGameList>,
    pagination_bar: String,
}

#[post("/media/game?<page>")]
pub async fn user_media_game(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_media_game::mk_lib_database_media_game_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let game_list = mk_lib_database_media_game::mk_lib_database_media_game_read(&sqlx_pool, String::new(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_game", &TemplateMediaGameContext {
        template_data: game_list,
        pagination_bar: pagination_html,
    })
}

#[post("/media/game_detail/<guid>")]
pub async fn user_media_game_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: User, guid: rocket::serde::uuid::Uuid) -> Template {
        let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
        Template::render("bss_user/media/bss_user_media_game_detail", tera::Context::new().into_json())
}

/*
@blueprint_user_game.route('/user_game', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game.html')
@common_global.auth.login_required
pub async fn url_bp_user_game(request):
    """
    Display game page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'media_games'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_game',
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


@blueprint_user_game.route('/user_game_detail/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_game_detail(request, guid):
    """
    Display game detail page
    """
    return {}

 */