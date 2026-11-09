use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_media_update_metadata_guid(
    sqlx_pool: &sqlx::PgPool,
    mm_media_guid: &i32,
    mm_metadata_guid: Uuid,
    mm_download_uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_media set mm_media_metadata_guid = $1
        where mm_media_guid = $2"#,
    )
    .bind(mm_metadata_guid)
    .bind(mm_media_guid)
    .execute(&mut *transaction)
    .await?;
    if mm_download_uuid != &uuid::Uuid::nil() {
        sqlx::query(r#"delete from mm_metadata_download_que where mm_download_guid = $1"#)
            .bind(mm_download_uuid)
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_media_unmatched_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_media
        where mm_media_metadata_guid is NULL"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaUnmatchedList {
    pub mm_media_guid: uuid::Uuid,
    pub mm_media_path: String,
}

pub async fn mk_lib_database_media_unmatched_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaUnmatchedList>, sqlx::Error> {
    let table_rows: Vec<DBMediaUnmatchedList> = sqlx::query_as(
        r#"select mm_media_guid,
        mm_media_path from mm_media
        where mm_media_metadata_guid is NULL
        order by mm_media_path offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_matched_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_media
        where mm_media_metadata_guid is not NULL"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_known_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_media"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaKnownList {
    pub mm_media_path: String,
}

pub async fn mk_lib_database_media_known(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaKnownList>, sqlx::Error> {
    let table_rows: Vec<DBMediaKnownList> = sqlx::query_as(
        r#"select mm_media_path
        from mm_media where mm_media_guid
        order by mm_media_path offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_insert(
    sqlx_pool: &sqlx::PgPool,
    mm_media_guid: Uuid,
    mm_media_class_enum: i16,
    mm_media_path: &str,
    mm_media_metadata_guid: Option<Uuid>,
    mm_media_ffprobe_json: serde_json::Value,
    mm_media_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_media (mm_media_guid, mm_media_class_enum,
         mm_media_path, mm_media_metadata_guid, mm_media_ffprobe_json, mm_media_json)
         values ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(mm_media_guid)
    .bind(mm_media_class_enum)
    .bind(mm_media_path)
    .bind(mm_media_metadata_guid)
    .bind(mm_media_ffprobe_json)
    .bind(mm_media_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_media_duplicate_detail_count(
    sqlx_pool: &sqlx::PgPool,
    mm_metadata_guid: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_media
        where mm_media_metadata_guid = $1"#,
    )
    .bind(mm_metadata_guid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_duplicate_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    // TODO technically this will "dupe" things like subtitles atm
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from (select mm_media_metadata_guid
        from mm_media where mm_media_metadata_guid is not null
        group by mm_media_metadata_guid HAVING count(*) > 1) as total"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_path_by_uuid(
    sqlx_pool: &sqlx::PgPool,
    mm_media_guid: Uuid,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as(
        r#"select mm_media_path from mm_media
        where mm_media_guid = $1"#,
    )
    .bind(mm_media_guid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaDuplicateList {
    mm_media_metadata_guid: uuid::Uuid,
    mm_media_name: String,
    mm_count: i64,
}

pub async fn mk_lib_database_media_duplicate(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaDuplicateList>, sqlx::Error> {
    // TODO technically this will "dupe" things like subtitles atm
    let table_rows: Vec<DBMediaDuplicateList> = sqlx::query_as(
        r#"select mm_media_metadata_guid,
        mm_media_name, count(*) as mm_count
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid is not null
        and mm_media_metadata_guid = mm_metadata_guid
        group by mm_media_metadata_guid,
        mm_media_name HAVING count(*) > 1 order by LOWER(mm_media_name)
        offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaDuplicateDetailList {
    mm_media_guid: uuid::Uuid,
    mm_media_path: String,
    mm_media_ffprobe_json: serde_json::Value,
}

pub async fn mk_lib_database_media_duplicate_detail(
    sqlx_pool: &sqlx::PgPool,
    mm_metadata_guid: Uuid,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaDuplicateDetailList>, sqlx::Error> {
    let table_rows: Vec<DBMediaDuplicateDetailList> = sqlx::query_as(
        r#"select mm_media_guid,
        mm_media_path, mm_media_ffprobe_json
        from mm_media where mm_media_guid
        in (select mm_media_guid from mm_media
        where mm_media_metadata_guid = $1 offset $2 limit $3"#,
    )
    .bind(mm_metadata_guid)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_image_path_by_uuid(
    sqlx_pool: &sqlx::PgPool,
    mm_media_guid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        r#"select mm_metadata_localimage_json
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_guid
        and mm_media_guid = $1"#,
    )
    .bind(mm_media_guid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_ffmpeg_update_by_uuid(
    sqlx_pool: &sqlx::PgPool,
    mm_media_guid: Uuid,
    ffmpeg_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_media set mm_media_ffprobe_json = $1
        where mm_media_guid = $2"#,
    )
    .bind(ffmpeg_json)
    .bind(mm_media_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_media_new_count(
    sqlx_pool: &sqlx::PgPool,
    days_old: i64,
) -> Result<i64, sqlx::Error> {
    let date_added = (Utc::now() - Duration::days(days_old))
        .format("%Y-%m-%d")
        .to_string();
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_media, mm_metadata_movie 
        where mm_media_metadata_guid = mm_metadata_movie_guid 
        and mm_media_json->>'DateAdded' >= $1"#,
    )
    .bind(date_added)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_new(
    sqlx_pool: &sqlx::PgPool,
    days_old: i64,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaKnownList>, sqlx::Error> {
    let date_added = (Utc::now() - Duration::days(days_old))
        .format("%Y-%m-%d")
        .to_string();
    let table_rows: Vec<DBMediaKnownList> = sqlx::query_as(
        r#"select mm_media_name,
         mm_media_guid,
         mm_media_class_guid
         from mm_media, mm_metadata_movie
         where mm_media_metadata_guid = mm_metadata_movie_guid
         and mm_media_json->>'DateAdded' >= $1
         order by LOWER(mm_media_name),
         mm_media_class_guid offset $2 limit $3"#,
    )
    .bind(date_added)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

/// Set the per-user rating/watch status for media.
///
/// A status of "watched" or "mismatch" stores boolean `true` under that
/// status name; any other status is stored as its string value under the
/// `Rating` key.
pub async fn mk_lib_database_media_rating_update(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
    user_id: i64,
    status_text: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        r#"select mm_media_json
        from mm_media
        where mm_media_guid = $1 for update"#,
    )
    .bind(media_guid)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some((mut media_json,)) = row else {
        transaction.rollback().await?;
        return Err(sqlx::Error::RowNotFound);
    };
    let (status_key, status_value) = if status_text == "watched" || status_text == "mismatch" {
        (status_text, serde_json::Value::Bool(true))
    } else {
        ("Rating", serde_json::Value::String(status_text.to_string()))
    };
    upsert_media_user_stat(&mut media_json, user_id, status_key, status_value);
    sqlx::query(
        r#"update mm_media set mm_media_json = $1
        where mm_media_guid = $2"#,
    )
    .bind(&media_json)
    .bind(media_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Set the per-user ffmpeg playback checkpoint for media
/// (so a user can pick up where they left off).
///
/// The Python original took an untyped `ffmpeg_time`; ported as `f64`
/// (seconds, fractional).
pub async fn mk_lib_database_media_watched_checkpoint_update(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
    user_id: i64,
    ffmpeg_time: f64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        r#"select mm_media_json
        from mm_media
        where mm_media_guid = $1 for update"#,
    )
    .bind(media_guid)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some((mut media_json,)) = row else {
        transaction.rollback().await?;
        return Err(sqlx::Error::RowNotFound);
    };
    upsert_media_user_stat(
        &mut media_json,
        user_id,
        "ffmpeg_checkpoint",
        serde_json::Value::from(ffmpeg_time),
    );
    sqlx::query(
        r#"update mm_media set mm_media_json = $1
        where mm_media_guid = $2"#,
    )
    .bind(&media_json)
    .bind(media_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Update the media json for the media guid.
pub async fn mk_lib_database_media_update_json(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
    media_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_media set mm_media_json = $1
        where mm_media_guid = $2"#,
    )
    .bind(media_json)
    .bind(media_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// All media matching a metadata guid and media class.
///
/// NOTE: the Python self-join `mm_media_metadata_guid = mm_metadata_guid` was
/// a tautology against a non-existent column; ported as
/// `= mm_metadata_movie_guid`. `mm_media_name` / `mm_media_class_guid`
/// follow the live `mk_lib_database_media_new` idiom; the schema dump only
/// lists `mm_media_class_enum` (confirm against the live database). The
/// Python `media_class_uuid` param is bound as the `DLMediaType` i16,
/// matching the live `mk_lib_database_media_movie_genre_count` bind.
pub async fn mk_lib_database_media_by_metadata_guid(
    sqlx_pool: &sqlx::PgPool,
    metadata_guid: Uuid,
    media_class: i16,
) -> Result<Vec<(String, Uuid)>, sqlx::Error> {
    let rows: Vec<(String, Uuid)> = sqlx::query_as(
        r#"select mm_media_name,
        mm_media_guid
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
        and mm_media_metadata_guid = $1
        and mm_media_class_guid = $2"#,
    )
    .bind(metadata_guid)
    .bind(media_class)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMetadataBothList {
    pub mm_media_name: String,
    pub mm_media_metadata_guid: Option<Uuid>,
    pub mm_media_ffprobe_json: serde_json::Value,
    pub mm_media_json: serde_json::Value,
    pub mm_metadata_movie_json: serde_json::Value,
    pub mm_metadata_movie_localimage_json: serde_json::Value,
    pub mm_metadata_movie_media_id: i32,
}

/// Read media and its movie metadata together by media guid.
///
/// NOTE: the Python original used the short names `mm_metadata_json` /
/// `mm_metadata_localimage_json` / `mm_metadata_media_id` (no such movie
/// columns); ported to `mm_metadata_movie_*`. The Python self-join
/// `mm_media_metadata_guid = mm_metadata_guid` was a tautology against a
/// non-existent column; ported as `= mm_metadata_movie_guid`.
pub async fn mk_lib_database_media_read_media_metadata_both(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
) -> Result<Option<DBMediaMetadataBothList>, sqlx::Error> {
    let row: Option<DBMediaMetadataBothList> = sqlx::query_as(
        r#"select mm_media_name,
        mm_media_metadata_guid,
        mm_media_ffprobe_json,
        mm_media_json,
        mm_metadata_movie_json,
        mm_metadata_movie_localimage_json,
        mm_metadata_movie_media_id
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
        and mm_media_guid = $1"#,
    )
    .bind(media_guid)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row)
}

/// The first media metadata guid whose path starts with the given path
/// (used for trailer and extras matching, since the media live within the
/// root directory, not at it).
pub async fn mk_lib_database_media_read_path_like(
    sqlx_pool: &sqlx::PgPool,
    media_path: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let row: Option<(Option<Uuid>,)> = sqlx::query_as(
        r#"select mm_media_metadata_guid
        from mm_media
        where mm_media_path like $1
        and mm_media_metadata_guid is not NULL limit 1"#,
    )
    .bind(format!("{media_path}%"))
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.and_then(|row| row.0))
}

/// All media guids and ffprobe json sharing the metadata of the given media
/// and media class.
///
/// NOTE: the Python original comma-joined `mm_metadata_movie` with no
/// predicate (relies on `distinct`); ported with the proper join
/// `mm_media_metadata_guid = mm_metadata_movie_guid`. `mm_media_class_guid`
/// follows the live `mk_lib_database_media_new` idiom; the schema dump only
/// lists `mm_media_class_enum` (confirm against the live database). The
/// Python `media_class_uuid` param is bound as the `DLMediaType` i16,
/// matching the live `mk_lib_database_media_movie_genre_count` bind.
pub async fn mk_lib_database_media_ffprobe_all_media_guid(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Uuid,
    media_class: i16,
) -> Result<Vec<(Uuid, serde_json::Value)>, sqlx::Error> {
    let rows: Vec<(Uuid, serde_json::Value)> = sqlx::query_as(
        r#"select distinct mm_media_guid,
        mm_media_ffprobe_json
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
        and mm_media_metadata_guid =
        (select mm_media_metadata_guid
        from mm_media where mm_media_guid = $1)
        and mm_media_class_guid = $2"#,
    )
    .bind(media_guid)
    .bind(media_class)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

/// Set `UserStats[user_id][stat_key]` in the media json, creating the
/// `UserStats` and per-user objects when missing.
fn upsert_media_user_stat(
    media_json: &mut serde_json::Value,
    user_id: i64,
    stat_key: &str,
    stat_value: serde_json::Value,
) {
    let user_key = user_id.to_string();
    if !media_json
        .get("UserStats")
        .is_some_and(serde_json::Value::is_object)
    {
        media_json["UserStats"] = serde_json::json!({});
    }
    if !media_json
        .get("UserStats")
        .and_then(|stats| stats.get(user_key.as_str()))
        .is_some_and(serde_json::Value::is_object)
    {
        media_json["UserStats"][user_key.as_str()] = serde_json::json!({});
    }
    if let Some(user_stats) = media_json
        .get_mut("UserStats")
        .and_then(|stats| stats.get_mut(user_key.as_str()))
        .and_then(serde_json::Value::as_object_mut)
    {
        user_stats.insert(stat_key.to_string(), stat_value);
    }
}
