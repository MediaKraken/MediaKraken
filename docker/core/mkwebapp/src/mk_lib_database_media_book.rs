use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaBookList {
	mm_metadata_book_guid: uuid::Uuid,
	mm_metadata_book_name: String,
}

pub async fn mk_lib_database_media_book_read(pool: &sqlx::PgPool,
                                              search_value: String,
                                              offset: i32, limit: i32)
                                              -> Result<Vec<DBMediaBookList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book, mm_media \
            where mm_media_metadata_guid = mm_metadata_book_guid \
            and mm_metadata_book_name % $1 \
            order by LOWER(mm_metadata_book_name) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book, mm_media \
            where mm_media_metadata_guid = mm_metadata_book_guid \
            order by LOWER(mm_metadata_book_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMediaBookList> = select_query
        .map(|row: PgRow| DBMediaBookList {
            mm_metadata_book_guid: row.get("mm_metadata_book_guid"),
            mm_metadata_book_name: row.get("mm_metadata_book_name"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_book_count(pool: &sqlx::PgPool,
                                              search_value: String)
                                              -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid \
            and mm_metadata_book_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}