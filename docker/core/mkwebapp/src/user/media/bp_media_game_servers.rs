use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_game_servers.rs"]
mod mk_lib_database_game_servers;

#[derive(Serialize)]
struct TemplateMediaGameServersContext<> {
    template_data: Vec<mk_lib_database_game_servers::DBGameServerList>,
    pagination_bar: String,
}

#[get("/media/game_servers?<page>")]
pub async fn user_media_game_servers(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_game_servers::mk_lib_database_game_server_count(&sqlx_pool, "".to_string()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let game_server_list = mk_lib_database_game_servers::mk_lib_database_game_server_read(&sqlx_pool, "".to_string(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_game_server", &TemplateMediaGameServersContext {
        template_data: game_server_list,
        pagination_bar: pagination_html,
    })
}

#[get("/media/game_servers_detail/<guid>")]
pub async fn user_media_game_servers_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, guid: Uuid) -> Template {
    Template::render("bss_user/media/bss_user_media_game_server_detail", {})
}

/*
@blueprint_user_game_servers.route('/user_game_server', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game_server.html')
@common_global.auth.login_required
pub async fn url_bp_user_game_server_list(request):
    """
    Display game server page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_game_server',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_game_dedicated_servers',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_game_server_list(offset,
                                                                    int(request.ctx.session[
                                                                            'per_page']),
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data,
        'pagination_bar': pagination,
    }

 */