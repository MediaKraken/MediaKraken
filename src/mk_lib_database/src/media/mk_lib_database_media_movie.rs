use chrono::{Duration, Utc};
use mk_lib_common::mk_lib_common_enum_media_type;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_media_movie_genre_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query(
        r#"select mm_metadata_json->'genres' as gen,
        count(mm_metadata_json->'genres') as gen_count
        from ((select distinct on (mm_media_metadata_guid)
        mm_metadata_json from mm_media, mm_metadata_movie
        where mm_media_class_guid = $1
        and mm_media_metadata_guid = mm_metadata_guid) union (select distinct
        on (mmr_media_metadata_guid) mm_metadata_json from mm_media_remote,
        mm_metadata_movie where mmr_media_class_guid = $2
        and mmr_media_metadata_guid = mm_metadata_guid))
        as temp group by gen"#,
    )
    .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
    .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

pub async fn mk_lib_database_media_movie_random(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(Uuid, Uuid), sqlx::Error> {
    let row: (Uuid, Uuid) = sqlx::query_as(
        r#"select mm_metadata_guid, mm_media_guid
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_guid
        and random() < 0.01 limit 1"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMovieList {
    pub mm_media_movie_video_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_movie_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaMovieList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_media_guid as mm_media_movie_video_guid
            from (select distinct on (mm_metadata_movie_guid) mm_media_guid,
            mm_metadata_movie_guid,
            mm_metadata_movie_name
            from mm_media,
            mm_metadata_movie where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid
            and mm_metadata_movie_name % $2
            order by mm_metadata_movie_guid, mm_media_path) as temp
            order by LOWER(temp.mm_metadata_movie_name),
            temp.mm_metadata_movie_guid
            offset $3 limit $4"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_media_guid as mm_media_movie_video_guid
            from (select distinct on (mm_metadata_movie_guid) mm_media_guid,
            mm_metadata_movie_guid,
            mm_metadata_movie_name
            from mm_media,
            mm_metadata_movie where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid
            order by mm_metadata_movie_guid, mm_media_path) as temp
            order by LOWER(temp.mm_metadata_movie_name),
            temp.mm_metadata_movie_guid
            offset $2 limit $3"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_media_movie_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from (select distinct mm_metadata_movie_guid
            from mm_media,
            mm_metadata_movie where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid
            and mm_metadata_movie_name % $2) as temp"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from (select distinct mm_metadata_movie_guid
            from mm_media,
            mm_metadata_movie where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid) as temp"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMovieMetadataBothList {
    pub mm_media_ffprobe_json: serde_json::Value,
    pub mm_metadata_movie_json: serde_json::Value,
    pub mm_metadata_movie_localimage_json: serde_json::Value,
}

/// Read in movie metadata and ffprobe by media guid.
///
/// NOTE: the Python original used the short names `mm_metadata_json` /
/// `mm_metadata_localimage_json` (no such movie columns); ported to
/// `mm_metadata_movie_*`. The Python self-join
/// `mm_media_metadata_guid = mm_metadata_guid` was a tautology against a
/// non-existent column; ported as `= mm_metadata_movie_guid`.
pub async fn mk_lib_database_media_movie_read_media_metadata_both(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
) -> Result<Option<DBMediaMovieMetadataBothList>, sqlx::Error> {
    let row: Option<DBMediaMovieMetadataBothList> = sqlx::query_as(
        r#"select mm_media_ffprobe_json,
        mm_metadata_movie_json,
        mm_metadata_movie_localimage_json
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
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
/// `mm_media_guid`) and selected the non-existent `mm_metadata_guid` from
/// `mm_media` (ported as `mm_media_metadata_guid`); both fixed. The Python
/// loop over the ffprobe streams built local audio/subtitle lists that were
/// never returned (the function always returned an empty list); that dead
/// loop and the ES logging were dropped.
pub async fn mk_lib_database_media_movie_read_media_list_by_uuid(
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMovieListRow {
    pub mm_media_name: String,
    pub mm_media_guid: Uuid,
    pub mm_metadata_movie_user_json: serde_json::Value,
    pub mm_poster: serde_json::Value,
    pub mm_media_path: Option<String>,
    pub mm_metadata_movie_json: serde_json::Value,
}

/// The web media movie list.
///
/// `recent_addition` restricts to media added within the last 7 days;
/// `list_genre` is "all" (no genre filter) or a genre name matched against
/// the first genre in the movie json; `include_remote` unions in the
/// `mm_media_remote` rows.
///
/// NOTE: the Python original had a `group_collection` branch; all of its
/// variants were `select 1` stubs or referenced non-existent columns
/// (`nullb`, `mm_metadata_collection_name`, `xxxx`); not ported. The
/// `search_text` parameter was accepted but never used; not ported. The
/// remote branch selected `mmr_media_name`, which does not exist on
/// `mm_media_remote`; reconstructed as `mm_metadata_movie_name` (available
/// via the metadata join). Some of the Python remote-union variants had
/// unbalanced parentheses and an invalid `%6` placeholder; the builder
/// emits balanced SQL. The Python self-join
/// `mm_media_metadata_guid = mm_metadata_guid` was a tautology against a
/// non-existent column; ported as `= mm_metadata_movie_guid`. Short names
/// `mm_metadata_user_json` / `mm_metadata_localimage_json` /
/// `mm_metadata_json` ported to `mm_metadata_movie_*`.
pub async fn mk_lib_database_media_movie_list(
    sqlx_pool: &sqlx::PgPool,
    class_enum: i16,
    recent_addition: bool,
    list_genre: &str,
    include_remote: bool,
    offset: Option<i64>,
    limit: i64,
) -> Result<Vec<DBMediaMovieListRow>, sqlx::Error> {
    let date_added = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let has_genre = list_genre != "all";
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(String::new());
    query.push(
        "select * from (select distinct on (mm_media_metadata_guid) mm_media_name, \
         mm_media_guid, mm_metadata_movie_user_json, \
         mm_metadata_movie_localimage_json->'Poster' as mm_poster, mm_media_path, \
         mm_metadata_movie_json from mm_media, mm_metadata_movie \
         where mm_media_class_guid = ",
    );
    query.push_bind(class_enum);
    query.push(" and mm_media_metadata_guid = mm_metadata_movie_guid");
    if recent_addition {
        query.push(" and mm_media_json->>'DateAdded' >= ");
        query.push_bind(&date_added);
    }
    if has_genre {
        query.push(" and mm_metadata_movie_json->'genres'->0->'name' ? ");
        query.push_bind(list_genre);
    }
    if include_remote {
        query.push(
            " union (select distinct on (mmr_media_metadata_guid) \
             mm_metadata_movie_name, mmr_media_guid, mmr_media_json, \
             mm_metadata_movie_localimage_json->'Poster' as mm_poster, \
             NULL as mmr_media_path, mm_metadata_movie_json \
             from mm_media_remote, mm_metadata_movie \
             where mmr_media_class_guid = ",
        );
        query.push_bind(class_enum);
        query.push(" and mmr_media_metadata_guid = mm_metadata_movie_guid");
        if recent_addition {
            query.push(" and mmr_media_json->>'DateAdded' >= ");
            query.push_bind(&date_added);
        }
        if has_genre {
            query.push(" and mm_metadata_movie_json->'genres'->0->'name' ? ");
            query.push_bind(list_genre);
        }
        query.push(")");
    }
    query.push(
        ") as temp order by LOWER(mm_media_name), \
         mm_metadata_movie_json->>'release_date' asc",
    );
    if let Some(offset) = offset {
        query.push(" offset ");
        query.push_bind(offset);
        query.push(" limit ");
        query.push_bind(limit);
    }
    let rows: Vec<DBMediaMovieListRow> = query.build_query_as().fetch_all(sqlx_pool).await?;
    Ok(rows)
}

/// The web media movie list count.
///
/// `recent_addition` restricts to media added within the last 7 days;
/// `list_genre` is "all" (no genre filter) or a genre name matched against
/// the first genre in the movie json; `include_remote` unions in the
/// `mm_media_remote` rows.
///
/// NOTE: the Python `group_collection` branches were `select 1` stubs or
/// broken (`union (select count(*) from xxxx ...)`) SQL; not ported. The
/// Python remote branch selected the non-existent `mmr_metadata_guid`;
/// ported as `mmr_media_metadata_guid`. The short name `mm_metadata_guid`
/// is ported as `mm_metadata_movie_guid`. The Python self-join
/// `mm_media_metadata_guid = mm_metadata_guid` was a tautology against a
/// non-existent column; ported as `= mm_metadata_movie_guid`.
pub async fn mk_lib_database_media_movie_list_count(
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
        "select count(*) from (select distinct mm_metadata_movie_guid \
         from mm_media, mm_metadata_movie \
         where mm_media_class_guid = ",
    );
    query.push_bind(class_enum);
    query.push(" and mm_media_metadata_guid = mm_metadata_movie_guid");
    if recent_addition {
        query.push(" and mm_media_json->>'DateAdded' >= ");
        query.push_bind(&date_added);
    }
    if has_genre {
        query.push(" and mm_metadata_movie_json->'genres'->0->'name' ? ");
        query.push_bind(list_genre);
    }
    if include_remote {
        query.push(
            " union (select distinct mmr_media_metadata_guid \
             from mm_media_remote, mm_metadata_movie \
             where mmr_media_class_guid = ",
        );
        query.push_bind(class_enum);
        query.push(" and mmr_media_metadata_guid = mm_metadata_movie_guid");
        if recent_addition {
            query.push(" and mmr_media_json->>'DateAdded' >= ");
            query.push_bind(&date_added);
        }
        if has_genre {
            query.push(" and mm_metadata_movie_json->'genres'->0->'name' ? ");
            query.push_bind(list_genre);
        }
        query.push(")");
    }
    query.push(") as temp");
    let row: (i64,) = query.build_query_as().fetch_one(sqlx_pool).await?;
    Ok(row.0)
}
