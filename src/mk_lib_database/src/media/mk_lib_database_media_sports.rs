use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaSportsList {
    pub mm_metadata_music_video_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_sports_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaSportsList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(r#""#)
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as(r#""#)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

pub async fn mk_lib_database_media_sports_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(r#""#)
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#""#).fetch_one(sqlx_pool).await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaSportsListRow {
    pub mm_metadata_sports_name: String,
    pub mm_media_guid: Uuid,
    pub mm_metadata_sports_user_json: serde_json::Value,
    pub mm_metadata_sports_image_json: serde_json::Value,
    pub mm_media_path: Option<String>,
}

/// The web media sports list.
///
/// `recent_addition` restricts to media added within the last 7 days;
/// `list_genre` is "all" (no genre filter) or a genre name matched against
/// the first genre in the sports json; `include_remote` unions in the
/// `mm_media_remote` rows.
///
/// NOTE: the Python original had a `group_collection` branch; all of its
/// variants were `select 1` stubs or referenced non-existent columns
/// (`nullb`, `mm_metadata_collection_name`); not ported. The
/// `search_text` parameter was accepted but never used; not ported.
/// `mm_media_class_guid` / `mmr_media_class_guid` follow the live idiom;
/// the schema dump lists `mm_media_class_enum` / `mmr_media_class_enum`
/// (confirm against the live database).
pub async fn mk_lib_database_media_sports_list(
    sqlx_pool: &sqlx::PgPool,
    class_enum: i16,
    recent_addition: bool,
    list_genre: &str,
    include_remote: bool,
    offset: Option<i64>,
    limit: i64,
) -> Result<Vec<DBMediaSportsListRow>, sqlx::Error> {
    let date_added = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let has_genre = list_genre != "all";
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(String::new());
    query.push(
        "select * from (select distinct on (mm_media_metadata_guid) \
         mm_metadata_sports_name, mm_media_guid, \
         mm_metadata_sports_user_json, mm_metadata_sports_image_json, \
         mm_media_path from mm_media, mm_metadata_sports \
         where mm_media_class_guid = ",
    );
    query.push_bind(class_enum);
    query.push(" and mm_media_metadata_guid = mm_metadata_sports_guid");
    if recent_addition {
        query.push(" and mm_media_json->>'DateAdded' >= ");
        query.push_bind(&date_added);
    }
    if has_genre {
        query.push(" and mm_metadata_sports_json->'genres'->0->'name' ? ");
        query.push_bind(list_genre);
    }
    if include_remote {
        query.push(
            " union (select distinct on (mmr_media_metadata_guid) \
             mm_metadata_sports_name, mmr_media_guid, mmr_media_json, \
             mm_metadata_sports_image_json, NULL as mmr_media_path \
             from mm_media_remote, mm_metadata_sports \
             where mmr_media_class_guid = ",
        );
        query.push_bind(class_enum);
        query.push(" and mmr_media_metadata_guid = mm_metadata_sports_guid");
        if recent_addition {
            query.push(" and mmr_media_json->>'DateAdded' >= ");
            query.push_bind(&date_added);
        }
        if has_genre {
            query.push(" and mm_metadata_sports_json->'genres'->0->'name' ? ");
            query.push_bind(list_genre);
        }
        query.push(")");
    }
    query.push(") as temp order by LOWER(mm_metadata_sports_name)");
    if let Some(offset) = offset {
        query.push(" offset ");
        query.push_bind(offset);
        query.push(" limit ");
        query.push_bind(limit);
    }
    let rows: Vec<DBMediaSportsListRow> = query.build_query_as().fetch_all(sqlx_pool).await?;
    Ok(rows)
}

/// The web media sports list count.
///
/// `recent_addition` restricts to media added within the last 7 days;
/// `list_genre` is "all" (no genre filter) or a genre name matched against
/// the first genre in the sports json; `include_remote` unions in the
/// `mm_media_remote` rows.
///
/// NOTE: the Python `group_collection` branches were `select 1` stubs or
/// broken (`union (select count(*) from xxxx ...)`) SQL; not ported. One
/// Python remote variant selected the non-existent `mmr_metadata_guid` and
/// read `mm_media_json` (the local column) from the remote branch; ported
/// as `mmr_media_metadata_guid` / `mmr_media_json` (matching the other
/// Python variants).
pub async fn mk_lib_database_media_sports_list_count(
    sqlx_pool: &sqlx::PgPool,
    class_enum: i16,
    recent_addition: bool,
    list_genre: &str,
    include_remote: bool,
) -> Result<i64, sqlx::Error> {
    let date_added = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let has_genre = list_genre != "all";
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(String::new());
    query.push(
        "select count(*) from (select distinct mm_metadata_sports_guid \
         from mm_media, mm_metadata_sports \
         where mm_media_class_guid = ",
    );
    query.push_bind(class_enum);
    query.push(" and mm_media_metadata_guid = mm_metadata_sports_guid");
    if recent_addition {
        query.push(" and mm_media_json->>'DateAdded' >= ");
        query.push_bind(&date_added);
    }
    if has_genre {
        query.push(" and mm_metadata_sports_json->'genres'->0->'name' ? ");
        query.push_bind(list_genre);
    }
    if include_remote {
        query.push(
            " union (select distinct mmr_media_metadata_guid \
             from mm_media_remote, mm_metadata_sports \
             where mmr_media_class_guid = ",
        );
        query.push_bind(class_enum);
        query.push(" and mmr_media_metadata_guid = mm_metadata_sports_guid");
        if recent_addition {
            query.push(" and mmr_media_json->>'DateAdded' >= ");
            query.push_bind(&date_added);
        }
        if has_genre {
            query.push(" and mm_metadata_sports_json->'genres'->0->'name' ? ");
            query.push_bind(list_genre);
        }
        query.push(")");
    }
    query.push(") as temp");
    let row: (i64,) = query.build_query_as().fetch_one(sqlx_pool).await?;
    Ok(row.0)
}

/// Find a random sports item (the Python original samples with
/// `random() < 0.01` rather than ordering by `random()`).
pub async fn mk_lib_database_media_sports_random(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Option<(Uuid, Uuid)>, sqlx::Error> {
    let row: Option<(Uuid, Uuid)> = sqlx::query_as(
        r#"select mm_metadata_sports_guid,
        mm_media_guid
        from mm_media, mm_metadata_sports
        where mm_media_metadata_guid = mm_metadata_sports_guid
        and random() < 0.01 limit 1"#,
    )
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row)
}

/// The sports media count by genre.
///
/// NOTE: the Python original had a malformed SQL string (unclosed quote
/// and a stray `b` token) and used the path
/// `Meta->themoviedb->Meta->genres`; ported with the top-level `'genres'`
/// path, matching the live `mk_lib_database_media_movie_genre_count` idiom
/// and the sports list genre filter. Like the live movie genre count, the
/// rows are returned as `Vec<PgRow>` since the shape follows the jsonb
/// `genres` array (drift-prone).
pub async fn mk_lib_database_media_sports_count_by_genre(
    sqlx_pool: &sqlx::PgPool,
    class_enum: i16,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query(
        r#"select jsonb_array_elements_text(mm_metadata_sports_json->'genres') as gen,
        count(mm_metadata_sports_json->'genres') as gen_count
        from ((select distinct on (mm_media_metadata_guid)
        mm_metadata_sports_json from mm_media, mm_metadata_sports
        where mm_media_class_guid = $1
        and mm_media_metadata_guid = mm_metadata_sports_guid) union (select distinct
        on (mmr_media_metadata_guid) mm_metadata_sports_json from mm_media_remote,
        mm_metadata_sports where mmr_media_class_guid = $2
        and mmr_media_metadata_guid = mm_metadata_sports_guid))
        as temp group by gen"#,
    )
    .bind(class_enum)
    .bind(class_enum)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaSportsMetadataBothList {
    pub mm_media_ffprobe_json: serde_json::Value,
    pub mm_metadata_sports_json: serde_json::Value,
    pub mm_metadata_sports_image_json: serde_json::Value,
}

/// Read in sports metadata and ffprobe by media guid.
pub async fn mk_lib_database_media_sports_read_media_metadata_both(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
) -> Result<Option<DBMediaSportsMetadataBothList>, sqlx::Error> {
    let row: Option<DBMediaSportsMetadataBothList> = sqlx::query_as(
        r#"select mm_media_ffprobe_json,
        mm_metadata_sports_json,
        mm_metadata_sports_image_json
        from mm_media, mm_metadata_sports
        where mm_media_metadata_guid = mm_metadata_sports_guid
        and mm_media_guid = $1"#,
    )
    .bind(media_guid)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row)
}

/// The ffprobe json of all media sharing the metadata of the given media.
///
/// NOTE: the Python original had typos `mm_media_guild` (ported as
/// `mm_media_guid`) and selected the non-existent `mm_metadata_sports_guid`
/// from `mm_media` (ported as `mm_media_metadata_guid`); both fixed. The
/// Python loop over the ffprobe streams built local audio/subtitle lists
/// that were never returned (the function always returned an empty list);
/// that dead loop and the ES logging were dropped.
pub async fn mk_lib_database_media_sports_read_media_list_by_uuid(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let rows: Vec<(serde_json::Value,)> = sqlx::query_as(
        r#"select mm_media_ffprobe_json
        from mm_media
        where mm_media_metadata_guid in (select mm_media_metadata_guid
        from mm_media where mm_media_guid = $1)"#,
    )
    .bind(media_guid)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows.into_iter().map(|row| row.0).collect())
}
