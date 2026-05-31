use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::{
    Extension,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
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
    page_title: Option<String>,
}

pub async fn user_sync(
    State(state): State<AppState>,
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
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
            mk_lib_database::mk_lib_database_sync::mk_lib_database_sync_count(&state.sqlx_pool_ro)
                .await?;
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/book".to_string(),
            None,
            pagination_count,
        )
        .await?;
        let sync_list = mk_lib_database::mk_lib_database_sync::mk_lib_database_sync_list(
            &state.sqlx_pool_ro,
            uuid::Uuid::nil(),
            db_offset,
            pagination_count,
        )
        .await?;
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
            page_title: Some("MediaKraken Sync".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
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
