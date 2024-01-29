use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_metadata_openlib_author_detail(
    sqlx_pool: &sqlx::PgPool,
    author_id: String,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        "select mm_openlib_author_json from mm_openlib_author \
        where mm_openlib_author_id = $1",
    )
    .bind(author_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_openlib_edition_detail(
    sqlx_pool: &sqlx::PgPool,
    edition_id: String,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        "select mm_openlib_edition_json from mm_openlib_edition \
        where mm_openlib_edition_id = $1",
    )
    .bind(edition_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_openlib_work_detail(
    sqlx_pool: &sqlx::PgPool,
    work_id: String,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        "select mm_openlib_work_json from mm_openlib_work \
        where mm_openlib_work_id = $1",
    )
    .bind(work_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaOpenLibWorkList {
    pub mm_openlib_work_id: String,
    pub mm_openlib_work_name: String,
}

pub async fn mk_lib_database_metadata_openlib_work_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaOpenLibWorkList>, sqlx::Error> {
    // TODO sort by release date
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_openlib_work_id, mm_openlib_work_json \
            from mm_openlib_work where mm_metadata_book_name % $1 \
            order by mm_openlib_work_json offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_openlib_work_id, mm_openlib_work_json \
            from mm_openlib_work order by mm_openlib_work_json \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaOpenLibWorkList> = select_query
        .map(|row: PgRow| DBMetaOpenLibWorkList {
            mm_openlib_work_id: row.get("mm_openlib_work_id"),
            mm_openlib_work_name: row.get("mm_openlib_work_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_openlib_work_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_openlib_work \
            where mm_openlib_work_json % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_openlib_work")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_openlib_work_id_by_isbn(
    sqlx_pool: &sqlx::PgPool,
    isbn: String,
    isbn13: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_openlib_work_id \
        from mm_openlib_work \
        where mm_metadata_book_isbn = $1 \
        or mm_metadata_book_isbn13 = $2",
    )
    .bind(isbn)
    .bind(isbn13)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_openlib_author_upsert(
    sqlx_pool: &sqlx::PgPool,
    author_id: &str,
    json_data: &str,
) -> Result<(), sqlx::Error> {
    let json_json: serde_json::Value = serde_json::from_str(json_data).unwrap();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_openlib_author (mm_openlib_author_id, \
        mm_openlib_author_json) \
        values ($1,$2) \
        ON CONFLICT(mm_openlib_author_id) DO UPDATE SET mm_openlib_author_json = $3",
    )
    .bind(author_id)
    .bind(&json_json)
    .bind(&json_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_openlib_edition_upsert(
    sqlx_pool: &sqlx::PgPool,
    edition_id: &str,
    json_data: &str,
) -> Result<(), sqlx::Error> {
    let json_json: serde_json::Value = serde_json::from_str(json_data).unwrap();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_openlib_edition (mm_openlib_edition_id, \
        mm_openlib_edition_json) \
        values ($1,$2) \
        ON CONFLICT(mm_openlib_edition_id) DO UPDATE SET mm_openlib_edition_json = $3",
    )
    .bind(edition_id)
    .bind(&json_json)
    .bind(&json_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_openlib_rating_upsert(
    sqlx_pool: &sqlx::PgPool,
    rating_id: &str,
    json_data: &str,
) -> Result<(), sqlx::Error> {
    let json_json: serde_json::Value = serde_json::from_str(json_data).unwrap();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_openlib_rating (mm_openlib_rating_id, \
        mm_openlib_rating_json) \
        values ($1,$2) \
        ON CONFLICT(mm_openlib_rating_id) DO UPDATE SET mm_openlib_rating_json = $3",
    )
    .bind(rating_id)
    .bind(&json_json)
    .bind(&json_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_openlib_work_upsert(
    sqlx_pool: &sqlx::PgPool,
    work_id: &str,
    json_data: &str,
) -> Result<(), sqlx::Error> {
    let json_json: serde_json::Value = serde_json::from_str(json_data).unwrap();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_openlib_work (mm_openlib_work_id, \
        mm_openlib_work_json) \
        values ($1,$2) \
        ON CONFLICT(mm_openlib_work_id) DO UPDATE SET mm_openlib_work_json = $3",
    )
    .bind(work_id)
    .bind(&json_json)
    .bind(&json_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_openlib_work_id_by_name(
    sqlx_pool: &sqlx::PgPool,
    work_name: String,
) -> Result<String, sqlx::Error> {
    // TODO can be more than one by name
    // TODO sort by release date
    let row: (String,) = sqlx::query_as(
        "select mm_openlib_work_id \
        from mm_openlib_work \
        where mm_openlib_work_json = $1",
    )
    .bind(work_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}
