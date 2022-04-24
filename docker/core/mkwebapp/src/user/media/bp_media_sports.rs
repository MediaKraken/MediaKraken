use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_sports.rs"]
mod mk_lib_database_media_sports;

#[derive(Serialize)]
struct TemplateMediaSportsContext<> {
    template_data: Vec<mk_lib_database_media_sports::DBMediaSportsList>,
    pagination_bar: String,
}

#[get("/media/sports?<page>")]
pub async fn user_media_sports(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_media_sports::mk_lib_database_media_sports_count(&sqlx_pool, "".to_string()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let sports_list = mk_lib_database_media_sports::mk_lib_database_media_sports_read(&sqlx_pool, "".to_string(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_sports", &TemplateMediaSportsContext {
        template_data: sports_list,
        pagination_bar: pagination_html,
    })
}

#[get("/media/sports_detail/<guid>")]
pub async fn user_media_sports_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, guid: Uuid) -> Template {
    Template::render("bss_user/media/bss_user_media_sports_detail", {})
}

/*
# list of spoting events
@blueprint_user_sports.route("/user_sports", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports(request):
    """
    Display sporting events page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_sports_list(
            common_global.DLMediaType.Sports.value,
            offset,
            int(request.ctx.session[
                    'per_page']),
            request.ctx.session[
                'search_text'], db_connection=db_connection):
        media.append((row_data['mm_metadata_sports_guid'],
                      row_data['mm_metadata_sports_name']))
    request.ctx.session['search_page'] = 'media_sports'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_sports',
                                                                      item_count=await request.app.db_functions.db_media_sports_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {'media': media,
            'pagination_bar': pagination,
            }


@blueprint_user_sports.route("/user_sports_detail/<guid>", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports_detail(request, guid):
    """
    Display sports detail page
    """
    # poster image
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_thesportsdb_select_by_guid(guid,
                                                                                   db_connection=db_connection)
    try:
        if json_metadata['LocalImages']['Poster'] != None:
            data_poster_image = json_metadata['LocalImages']['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if json_metadata['LocalImages']['Backdrop'] != None:
            data_background_image = json_metadata['LocalImages']['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    await request.app.db_pool.release(db_connection)
    return {
        'data': media_data,
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
    }

 */