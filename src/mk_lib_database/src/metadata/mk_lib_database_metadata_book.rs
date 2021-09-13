use uuid::Uuid;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_metadata_book_by_uuid(pool: &sqlx::PgPool, book_uuid: uuid::Uuid)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_book_json from mm_metadata_book \
        where mm_metadata_book_guid = $1")
        .bind(book_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_metadata_book_read(pool: &sqlx::PgPool,
                                                search_value: String,
                                                offset: i32, limit: i32)
                                                -> Result<Vec<PgRow>, sqlx::Error> {
    // TODO sort by release date
    if search_value != "" {
        let rows = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book where mm_metadata_book_name % $1 \
            order by mm_metadata_book_name offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book order by mm_metadata_book_name \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

pub async fn mk_lib_database_metadata_book_count(pool: &sqlx::PgPool,
                                                 search_value: String)
                                                 -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book \
            where mm_metadata_book_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}
