use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

pub async fn mk_lib_database_metadata_exists_tv(
    sqlx_pool: &sqlx::PgPool,
    metadata_id: i32,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_metadata_tvshow where mm_metadata_media_tvshow_id = $1 limit 1) as found_record limit 1"#,
    )
    .bind(metadata_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaTVShowList {
    pub mm_metadata_tvshow_guid: uuid::Uuid,
    pub mm_metadata_tvshow_name: String,
    pub mm_metadata_tvshow_name_alt: Option<String>,
    pub air_date: serde_json::Value,
    pub image_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_tv_read(
    sqlx_pool: &sqlx::PgPool,
    starts_with: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaTVShowList>, sqlx::Error> {
    if !starts_with.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_tvshow_guid,
            mm_metadata_tvshow_name,
            mm_metadata_tvshow_name_alt,
            mm_metadata_tvshow_json->'first_air_date' as air_date,
            mm_metadata_tvshow_localimage_json->'Poster' as image_json
            from mm_metadata_tvshow
            where (
                ($1 = '#' AND left(lower(mm_metadata_tvshow_name), 1) !~ '^[a-z0-9]$')
                OR ($1 <> '#' AND (
                    lower(mm_metadata_tvshow_name) LIKE lower($1) || '%'
                    OR lower(coalesce(mm_metadata_tvshow_name_alt, '')) LIKE lower($1) || '%'
                ))
            )
            order by lower(mm_metadata_tvshow_name), mm_metadata_tvshow_json->'first_air_date'
            offset $2 limit $3"#,
        )
        .bind(&starts_with)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_tvshow_guid, mm_metadata_tvshow_name, mm_metadata_tvshow_name_alt, mm_metadata_tvshow_json->'first_air_date' as air_date, mm_metadata_tvshow_localimage_json->'Poster' as image_json from mm_metadata_tvshow order by LOWER(mm_metadata_tvshow_name), mm_metadata_tvshow_json->'first_air_date' offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_tv_count(
    sqlx_pool: &sqlx::PgPool,
    starts_with: String,
) -> Result<i64, sqlx::Error> {
    if !starts_with.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*)
            from mm_metadata_tvshow
            where (
                ($1 = '#' AND left(lower(mm_metadata_tvshow_name), 1) !~ '^[a-z0-9]$')
                OR ($1 <> '#' AND (
                    lower(mm_metadata_tvshow_name) LIKE lower($1) || '%'
                    OR lower(coalesce(mm_metadata_tvshow_name_alt, '')) LIKE lower($1) || '%'
                ))
            )"#,
        )
        .bind(starts_with)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_tvshow"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_tv_insert(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
    series_id: i32,
    data_json: &serde_json::Value,
    data_image_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut original_name = None;
    if !data_json["original_name"].is_null() && data_json["name"] != data_json["original_name"] {
        original_name = Some(data_json["original_name"].as_str().unwrap_or(""));
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_tvshow (mm_metadata_tvshow_guid, mm_metadata_media_tvshow_id, mm_metadata_tvshow_name, mm_metadata_tvshow_name_alt, mm_metadata_tvshow_json, mm_metadata_tvshow_localimage_json) values ($1,$2,$3,$4,$5,$6)"#,
    )
    .bind(uuid_id)
    .bind(series_id)
    .bind(data_json["name"].to_string())
    .bind(original_name)
    .bind(data_json)
    .bind(data_image_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_tv_status(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
    key: String,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: (serde_json::Value,) = sqlx::query_as(
        r#"select mm_metadata_tv_user_json from mm_metadata_tv where mm_metadata_tv_guid = $1"#,
    )
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await?;
    // extract user json, update status, update
    let mut user_json: serde_json::Value = row.0;
    let user_id = user_id.to_string();
    if user_json["UserStats"][&user_id].is_null() {
        user_json["UserStats"][&user_id] = serde_json::json!({"Rating": false, "Watched": false, "Requested": false, "Queue": false});
    }
    // TODO set the "keys"
    sqlx::query(
        r#"update mm_metadata_tv set mm_metadata_tv_user_json = $1 where mm_metadata_tv_guid = $2"#,
    )
    .bind(user_json)
    .bind(uuid_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
