use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_home_media.rs"]
mod mk_lib_database_media_home_media;

#[derive(Serialize)]
struct TemplateMediaHomeContext<> {
    template_data: Vec<mk_lib_database_media_home_media::DBMediaHomeMediaList>,
    pagination_bar: String,
}

#[get("/media/home_media?<page>")]
pub async fn user_media_home_media(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_media_home_media::mk_lib_database_media_home_media_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let home_list = mk_lib_database_media_home_media::mk_lib_database_media_home_media_read(&sqlx_pool, String::new(), 0 ,30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_home_movie", &TemplateMediaHomeContext {
        template_data: home_list,
        pagination_bar: pagination_html,
    })
}

#[derive(Serialize)]
struct TemplateMediaHomeDetailContext<> {
    template_data: serde_json::Value,
}

#[get("/media/home_media_detail/<guid>")]
pub async fn user_media_home_media_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: User, guid: rocket::serde::uuid::Uuid) -> Template {
        let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
        Template::render("bss_user/media/bss_user_media_home_movie_detail", tera::Context::new().into_json())
}


/*
@blueprint_user_home_media.route('/user_home_media', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_home_movie.html')
@common_global.auth.login_required
pub async fn url_bp_user_home_media_list(request):
    """
    Display home page for home media
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_media_movie_list(
        class_guid=common_global.DLMediaType.Movie_Home.value,
        list_type=None,
        list_genre='All',
        list_limit=int(request.ctx.session[
                           'per_page']),
        group_collection=False,
        offset=offset,
        include_remote=False,
        search_text=request.ctx.session[
            'search_text'], db_connection=db_connection)
    # pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
    #                                                                   url='/user/user_home_media',
    #                                                                   item_count=await request.app.db_functions.db_meta_game_system_list_count(
    #                                                                       db_connection),
    #                                                                   client_items_per_page=
    #                                                                   int(request.ctx.session[
    #                                                                           'per_page']),
    #                                                                   format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data
    }


@blueprint_user_home_media.route('/user_home_media_detail/<guid>')
@common_global.jinja_template.template('bss_user/media/bss_user_media_home_movie_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_home_media_detail(request, guid):
    """
    Display home page for home media
    """
    return {}

 */