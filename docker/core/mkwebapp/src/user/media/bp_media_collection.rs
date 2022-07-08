use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_collection.rs"]
mod mk_lib_database_metadata_collection;

#[derive(Serialize)]
struct TemplateMediaCollectionContext {
    template_data: Vec<mk_lib_database_metadata_collection::DBMetaCollectionList>,
    pagination_bar: String,
}

#[get("/media/collection/<page>")]
pub async fn user_media_collection(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_collection::mk_lib_database_metadata_collection_count(
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
        "/user/media/collection".to_string(),
    )
    .await
    .unwrap();
    let collection_list =
        mk_lib_database_metadata_collection::mk_lib_database_metadata_collection_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_movie_collection",
        &TemplateMediaCollectionContext {
            template_data: collection_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMediaCollectionDetailContext {
    template_data: serde_json::Value,
}

#[get("/media/collection_detail/<guid>")]
pub async fn user_media_collection_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_movie_collection_detail",
        tera::Context::new().into_json(),
    )
}

/*

@blueprint_user_media_collection.route('/user_media_movie_collection', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_movie_collection.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_movie_collection(request):
    """
    Display movie collection metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_collection_list(offset,
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      request.ctx.session[
                                                                          'search_text'],
                                                                      db_connection):
        if 'Poster' in row_data['mm_metadata_collection_imagelocal_json']:
            media.append((row_data['mm_metadata_collection_guid'],
                          row_data['mm_metadata_collection_name'],
                          row_data['mm_metadata_collection_imagelocal_json']['Poster']))
        else:
            media.append((row_data['mm_metadata_collection_guid'],
                          row_data['mm_metadata_collection_name'], None))
    request.ctx.session['search_page'] = 'meta_movie_collection'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_media_movie_collection',
                                                                      item_count=await request.app.db_functions.db_collection_list_count(
                                                                          search_value=
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media,
        'pagination_bar': pagination,
    }


@blueprint_user_media_collection.route('/user_media_movie_collection_detail/<guid>')
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_movie_collection_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_movie_collection_detail(request, guid):
    """
    Display movie collection metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    data_metadata = await request.app.db_functions.db_collection_read_by_guid(guid,
                                                                              db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    json_metadata = data_metadata['mm_metadata_collection_json']
    json_imagedata = data_metadata['mm_metadata_collection_imagelocal_json']
    # poster image
    try:
        if json_imagedata['Poster'] != None:
            data_poster_image = json_imagedata['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if json_imagedata['Backdrop'] != None:
            data_background_image = json_imagedata['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    return {
        'data_name': json_metadata['name'],
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
        'json_metadata': json_metadata,
    }

 */
