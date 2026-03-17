use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub async fn mk_lib_database_report_known_media_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_media"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBReportKnownMediaList {
    pub mm_media_path: String,
    pub mm_media_class_enum: i16,
    pub mm_media_json_added: DateTime<Utc>,
}

pub async fn mk_lib_database_report_known_media_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBReportKnownMediaList>, sqlx::Error> {
    let table_rows: Vec<DBReportKnownMediaList> = sqlx::query_as(
        r#"select mm_media_path, mm_media_class_enum, (mm_media_json->>'Added')::timestamptz as mm_media_json_added from mm_media order by mm_media_json_added desc offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}
