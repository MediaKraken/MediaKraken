use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_book_detail(
    sqlx_pool: &sqlx::PgPool,
    book_uuid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        r#"select mm_metadata_book_json from mm_metadata_book
        where mm_metadata_book_guid = $1"#,
    )
    .bind(book_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaBookList {
    pub mm_metadata_book_guid: uuid::Uuid,
    pub mm_metadata_book_name: String,
}

pub async fn mk_lib_database_metadata_book_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaBookList>, sqlx::Error> {
    // TODO sort by release date
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_book_guid, mm_metadata_book_name
            from mm_metadata_book where mm_metadata_book_name &@ $1
            offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_book_guid, mm_metadata_book_name
            from mm_metadata_book order by mm_metadata_book_name
            offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_book_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_book
            where mm_metadata_book_name &@ $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_book"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_book_guid_by_isbn(
    sqlx_pool: &sqlx::PgPool,
    isbn: String,
    isbn13: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select mm_metadata_book_guid
        from mm_metadata_book
        where mm_metadata_book_isbn = $1
        or mm_metadata_book_isbn13 = $2"#,
    )
    .bind(isbn)
    .bind(isbn13)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_book_insert(
    sqlx_pool: &sqlx::PgPool,
    json_data: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_book (mm_metadata_book_guid,
        mm_metadata_book_isbn,
        mm_metadata_book_isbn13,
        mm_metadata_book_name,
        mm_metadata_book_json)
        values ($1,$2,$3,$4,$5)"#,
    )
    .bind(new_guid)
    .bind(&json_data["data"][0]["isbn10"])
    .bind(&json_data["data"][0]["isbn13"])
    .bind(&json_data["data"][0]["title"])
    .bind(&json_data["data"][0])
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_book_guid_by_name(
    sqlx_pool: &sqlx::PgPool,
    book_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    // TODO can be more than one by name
    // TODO sort by release date
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select mm_metadata_book_guid
        from mm_metadata_book
        where mm_metadata_book_name = $1"#,
    )
    .bind(book_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Find a random book image of the requested type (e.g. "Cover").
///
/// The image type is embedded in a JSON path in the SQL, so it is restricted to
/// `[A-Za-z0-9_-]`; the Python original was injectable ("little bobby tables").
pub async fn mk_lib_database_metadata_book_image_random(
    sqlx_pool: &sqlx::PgPool,
    return_image_type: &str,
) -> Result<Option<(serde_json::Value, uuid::Uuid)>, sqlx::Error> {
    if !is_valid_image_type(return_image_type) {
        return Err(sqlx::Error::Configuration(
            format!("invalid image type: {return_image_type}").into(),
        ));
    }
    let sql = format!(
        r#"select mm_metadata_book_localimage->'Images'->>{return_image_type} as image_json,
        mm_metadata_book_guid
        from mm_media, mm_metadata_book
        where mm_media_metadata_guid = mm_metadata_book_guid
        and (mm_metadata_book_localimage->'Images'->>{return_image_type})::text != 'null'
        order by random() limit 1"#,
    );
    let row: Option<(serde_json::Value, uuid::Uuid)> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .fetch_optional(sqlx_pool)
        .await?;
    Ok(row)
}

/// True when the image type is safe to embed in a SQL JSON path.
fn is_valid_image_type(image_type: &str) -> bool {
    !image_type.is_empty()
        && image_type
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
