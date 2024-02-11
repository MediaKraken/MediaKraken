use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_sync.html")]
struct TemplateSyncContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_sync::DBSyncList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_sync(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let db_offset: i64 = (page * 30) - 30;
        let total_pages: i64 =
            mk_lib_database::mk_lib_database_sync::mk_lib_database_sync_count(&sqlx_pool)
                .await
                .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/book".to_string(),
        )
        .await
        .unwrap();
        let sync_list = mk_lib_database::mk_lib_database_sync::mk_lib_database_sync_list(
            &sqlx_pool,
            uuid::Uuid::nil(),
            db_offset,
            30,
        )
        .await
        .unwrap();
        let mut template_data_exists = false;
        if sync_list.len() > 0 {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateSyncContext {
            template_data: &sync_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*

@blueprint_user_sync.route('/user_sync_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_user_admin_sync_delete_page(request):
    """
    Display sync delete action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_sync_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_user_sync.route('/user_sync_edit/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/user_sync_edit.html')
@common_global.auth.login_required
pub async fn url_bp_user_sync_edit(request, guid):
    """
    Allow user to edit sync page
    """
    if request.method == 'POST':
        db_connection = await request.app.db_pool.acquire()
        sync_json = {'Type': request.form['target_type'],
                     'Media GUID': guid,
                     'Options': {'VContainer': request.form['target_container'],
                                 'VCodec': request.form['target_codec'],
                                 'Size': request.form['target_file_size'],
                                 'AudioChannels': request.form['target_audio_channels'],
                                 'ACodec': request.form['target_audio_codec'],
                                 'ASRate': request.form['target_sample_rate']},
                     'Priority': request.form['target_priority'],
                     'Status': 'Scheduled',
                     'Progress': 0}
        await request.app.db_functions.db_sync_insert(request.form['name'],
                                                      request.form['target_output_path'],
                                                      sync_json,
                                                      db_connection=db_connection)
        await request.app.db_pool.release(db_connection)
        return redirect(
            request.app.url_for('name_blueprint_user_movie.url_bp_user_movie_detail', guid=guid))
    form = BSSSyncEditForm(request, csrf_enabled=False)
    if form.validate_on_submit():
        pass
    else:
        flash_errors(form)
    return {
        'guid': guid,
        'form': form
    }

 */
