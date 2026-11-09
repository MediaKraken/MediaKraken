use serde_json::{Map, Value, json};

/// Search the local metadata store by trigram similarity.
///
/// Port of the old Python `db_search` ("Local" mode; the Python original had no
/// other search types). Each enabled section is a JSON array of
/// `{guid, name, sml}` objects, keyed by "Movie", "TVShow", "Album",
/// "Publication" and "Game". The Python original stringified each section with
/// `json.dumps`; this returns proper JSON arrays.
#[allow(clippy::too_many_arguments)]
pub async fn mk_lib_database_search(
    sqlx_pool: &sqlx::PgPool,
    search_string: &str,
    search_type: &str,
    search_movie: bool,
    search_tvshow: bool,
    search_album: bool,
    search_image: bool,
    search_publication: bool,
    search_game: bool,
) -> Result<Map<String, Value>, sqlx::Error> {
    let mut json_return_data: Map<String, Value> = Map::new();
    if search_type == "Local" {
        if search_movie {
            let rows: Vec<(uuid::Uuid, String, f64)> = sqlx::query_as(
                r#"SELECT mm_metadata_movie_guid,
                mm_metadata_movie_name,
                similarity(mm_metadata_movie_name, $1) AS sml
                FROM mm_metadata_movie
                WHERE mm_metadata_movie_name % $2
                ORDER BY sml DESC, LOWER(mm_metadata_movie_name)"#,
            )
            .bind(search_string)
            .bind(search_string)
            .fetch_all(sqlx_pool)
            .await?;
            json_return_data.insert(
                "Movie".to_string(),
                Value::Array(
                    rows.into_iter()
                        .map(|(guid, name, sml)| {
                            json!({
                                "mm_metadata_movie_guid": guid,
                                "mm_metadata_movie_name": name,
                                "sml": sml,
                            })
                        })
                        .collect(),
                ),
            );
        }
        if search_tvshow {
            let rows: Vec<(uuid::Uuid, String, f64)> = sqlx::query_as(
                r#"SELECT mm_metadata_tvshow_guid,
                mm_metadata_tvshow_name,
                similarity(mm_metadata_tvshow_name, $1) AS sml
                FROM mm_metadata_tvshow
                WHERE mm_metadata_tvshow_name % $2
                ORDER BY sml DESC, LOWER(mm_metadata_tvshow_name)"#,
            )
            .bind(search_string)
            .bind(search_string)
            .fetch_all(sqlx_pool)
            .await?;
            json_return_data.insert(
                "TVShow".to_string(),
                Value::Array(
                    rows.into_iter()
                        .map(|(guid, name, sml)| {
                            json!({
                                "mm_metadata_tvshow_guid": guid,
                                "mm_metadata_tvshow_name": name,
                                "sml": sml,
                            })
                        })
                        .collect(),
                ),
            );
        }
        if search_album {
            let rows: Vec<(uuid::Uuid, String, f64)> = sqlx::query_as(
                r#"SELECT mm_metadata_album_guid,
                mm_metadata_album_name,
                similarity(mm_metadata_album_name, $1) AS sml
                FROM mm_metadata_album
                WHERE mm_metadata_album_name % $2
                ORDER BY sml DESC, LOWER(mm_metadata_album_name)"#,
            )
            .bind(search_string)
            .bind(search_string)
            .fetch_all(sqlx_pool)
            .await?;
            json_return_data.insert(
                "Album".to_string(),
                Value::Array(
                    rows.into_iter()
                        .map(|(guid, name, sml)| {
                            json!({
                                "mm_metadata_album_guid": guid,
                                "mm_metadata_album_name": name,
                                "sml": sml,
                            })
                        })
                        .collect(),
                ),
            );
        }
        if search_image {
            // TODO image search (the Python original had no SQL here)
        }
        if search_publication {
            let rows: Vec<(uuid::Uuid, String, f64)> = sqlx::query_as(
                r#"SELECT mm_metadata_book_guid,
                mm_metadata_book_name,
                similarity(mm_metadata_book_name, $1) AS sml
                FROM mm_metadata_book
                WHERE mm_metadata_book_name % $2
                ORDER BY sml DESC, LOWER(mm_metadata_book_name)"#,
            )
            .bind(search_string)
            .bind(search_string)
            .fetch_all(sqlx_pool)
            .await?;
            json_return_data.insert(
                "Publication".to_string(),
                Value::Array(
                    rows.into_iter()
                        .map(|(guid, name, sml)| {
                            json!({
                                "mm_metadata_book_guid": guid,
                                "mm_metadata_book_name": name,
                                "sml": sml,
                            })
                        })
                        .collect(),
                ),
            );
        }
        if search_game {
            let rows: Vec<(uuid::Uuid, String, f64)> = sqlx::query_as(
                r#"SELECT gi_game_info_id,
                gi_game_info_name,
                similarity(gi_game_info_name, $1) AS sml
                FROM mm_metadata_game_software_info
                WHERE gi_game_info_name % $2
                ORDER BY sml DESC, LOWER(gi_game_info_name)"#,
            )
            .bind(search_string)
            .bind(search_string)
            .fetch_all(sqlx_pool)
            .await?;
            json_return_data.insert(
                "Game".to_string(),
                Value::Array(
                    rows.into_iter()
                        .map(|(guid, name, sml)| {
                            json!({
                                "gi_game_info_id": guid,
                                "gi_game_info_name": name,
                                "sml": sml,
                            })
                        })
                        .collect(),
                ),
            );
        }
    }
    Ok(json_return_data)
}
