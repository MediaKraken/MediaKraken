use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_book.rs"]
mod mk_lib_database_media_book;

#[derive(Serialize)]
struct TemplateMediaBookContext<> {
    template_data: Vec<mk_lib_database_media_book::DBMediaBookList>,
    pagination_bar: String,
}

#[get("/media/book/<page>")]
pub async fn user_media_book(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i64 = mk_lib_database_media_book::mk_lib_database_media_book_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let book_list = mk_lib_database_media_book::mk_lib_database_media_book_read(&sqlx_pool, String::new(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_book", &TemplateMediaBookContext {
        template_data: book_list,
        pagination_bar: pagination_html,
    })
}

#[derive(Serialize)]
struct TemplateMediaBookDetailContext<> {
    template_data: serde_json::Value,
}

#[get("/media/book_detail/<guid>")]
pub async fn user_media_book_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: User, guid: rocket::serde::uuid::Uuid) -> Template {
        let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
        Template::render("bss_user/media/bss_user_media_book_detail", tera::Context::new().into_json())
}

/*

@blueprint_user_periodical.route('/user_periodical', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_periodical.html')
@common_global.auth.login_required
pub async fn url_bp_user_periodical_list(request):
    """
    Display periodical page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'media_periodicals'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_media_book_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_media_book_list(offset,
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


@blueprint_user_periodical.route('/user_periodical_detail/<guid>', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_periodical_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_periodical_detail(request, guid):
    """
    Display periodical detail page
    """
    return {}

 */