use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::{Query, State};
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use serde::de::Deserializer;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

/// One search result row.
pub struct SearchHit {
    pub name: String,
    pub url: String,
    pub similarity: String,
}

/// One media type section of search results.
pub struct SearchSection {
    pub label: &'static str,
    pub hits: Vec<SearchHit>,
}

#[derive(Template)]
#[template(path = "bss_user/bss_user_media_search.html")]
pub struct UserSearchTemplate<'a> {
    pub page_title: Option<String>,
    pub template_search_query: &'a String,
    pub template_active_filters: &'a Vec<String>,
    pub template_search_error: &'a String,
    pub template_sections: &'a Vec<SearchSection>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    #[serde(default)]
    pub q: String,
    #[serde(default, deserialize_with = "deserialize_filter")]
    pub filter: Vec<String>,
}

fn deserialize_filter<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }

    match Option::<OneOrMany>::deserialize(deserializer)? {
        Some(OneOrMany::One(value)) => Ok(vec![value]),
        Some(OneOrMany::Many(values)) => Ok(values),
        None => Ok(Vec::new()),
    }
}

/// Map submitted filter values to the local media types to search. An empty
/// filter, "all", or the legacy topnavbar "available" value searches every
/// type; "video"/"audio" are accepted for compatibility with the old Python
/// form values.
fn filter_to_search_types(filters: &[String]) -> (bool, bool, bool, bool, bool) {
    let has = |name: &str| filters.iter().any(|f| f.eq_ignore_ascii_case(name));
    if filters.is_empty() || has("all") || has("available") {
        return (true, true, true, true, true);
    }
    (
        has("movie") || has("video"),
        has("tv") || has("tvshow") || has("video"),
        has("music") || has("album") || has("audio"),
        has("publication") || has("book"),
        has("game"),
    )
}

/// Pull one section out of the search result map; None when the section is
/// absent.
fn extract_section(
    label: &'static str,
    detail_route: &str,
    results: &serde_json::Map<String, serde_json::Value>,
    section_key: &str,
    guid_key: &str,
    name_key: &str,
) -> Option<SearchSection> {
    let rows = results.get(section_key)?;
    let arr = rows.as_array()?;
    let hits: Vec<SearchHit> = arr
        .iter()
        .filter_map(|row| {
            let guid = row.get(guid_key)?.as_str()?;
            let name = row.get(name_key)?.as_str()?;
            let sml = row
                .get("sml")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            Some(SearchHit {
                name: name.to_string(),
                url: format!("{detail_route}{guid}"),
                similarity: format!("{:.0}%", sml * 100.0),
            })
        })
        .collect();
    Some(SearchSection { label, hits })
}

pub async fn user_search(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(params): Query<SearchParams>,
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
        // Don't reset the selected type after a search: echo the submitted
        // filters back to the template so the checkboxes keep their state.
        // No filter at all means "search everything".
        let active_filters = if params.filter.is_empty() {
            vec!["all".to_string()]
        } else {
            params.filter.clone()
        };
        let query = params.q.trim().to_string();

        let mut search_error = String::new();
        let mut sections: Vec<SearchSection> = Vec::new();
        if !query.is_empty() {
            let (movie, tvshow, album, publication, game) = filter_to_search_types(&params.filter);
            match mk_lib_database::mk_lib_database_search::mk_lib_database_search(
                &state.sqlx_pool_ro,
                &query,
                "Local",
                movie,
                tvshow,
                album,
                // image: the lib has no SQL for local image search yet
                false,
                publication,
                game,
            )
            .await
            {
                Ok(results) => {
                    // Result links open the metadata detail pages, not the
                    // media pages; the search keys are the same uuids both
                    // sets of routes accept.
                    sections.extend(extract_section(
                        "Movies",
                        "/user/metadata/movie_detail/",
                        &results,
                        "Movie",
                        "mm_metadata_movie_guid",
                        "mm_metadata_movie_name",
                    ));
                    sections.extend(extract_section(
                        "TV Shows",
                        "/user/metadata/tv_detail/",
                        &results,
                        "TVShow",
                        "mm_metadata_tvshow_guid",
                        "mm_metadata_tvshow_name",
                    ));
                    // Music keeps the media endpoint: the metadata album
                    // route (/user/metadata/music_album/) is keyed by the
                    // numeric musicbrainz release id, not the local guid
                    // the search returns.
                    sections.extend(extract_section(
                        "Music",
                        "/user/media/music_detail/",
                        &results,
                        "Album",
                        "mm_metadata_album_guid",
                        "mm_metadata_album_name",
                    ));
                    sections.extend(extract_section(
                        "Books",
                        "/user/metadata/book_detail/",
                        &results,
                        "Publication",
                        "mm_metadata_book_guid",
                        "mm_metadata_book_name",
                    ));
                    sections.extend(extract_section(
                        "Games",
                        "/user/metadata/game_detail/",
                        &results,
                        "Game",
                        "gi_game_info_id",
                        "gi_game_info_name",
                    ));
                    sections.retain(|section| !section.hits.is_empty());
                }
                Err(error) => {
                    eprintln!("Local search failed for {query:?}: {error}");
                    search_error = "Search failed. Please try again.".to_string();
                }
            }
        }

        let template = UserSearchTemplate {
            page_title: Some("MediaKraken Search".to_string()),
            template_search_query: &query,
            template_active_filters: &active_filters,
            template_search_error: &search_error,
            template_sections: &sections,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
