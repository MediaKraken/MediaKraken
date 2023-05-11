use mk_lib_logging::mk_lib_logging;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_metadata_book_detail(
    sqlx_pool: &sqlx::PgPool,
    book_uuid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (serde_json::Value,) = sqlx::query_as(
        "select mm_metadata_book_json from mm_metadata_book \
        where mm_metadata_book_guid = $1",
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO sort by release date
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book where mm_metadata_book_name % $1 \
            order by mm_metadata_book_name offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book order by mm_metadata_book_name \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaBookList> = select_query
        .map(|row: PgRow| DBMetaBookList {
            mm_metadata_book_guid: row.get("mm_metadata_book_guid"),
            mm_metadata_book_name: row.get("mm_metadata_book_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_book_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_book \
            where mm_metadata_book_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_book")
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_metadata_book_guid \
        from mm_metadata_book \
        where mm_metadata_book_isbn = $1 \
        or mm_metadata_book_isbn13 = $2",
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_metadata_book (mm_metadata_book_guid, \
        mm_metadata_book_isbn, \
        mm_metadata_book_isbn13, \
        mm_metadata_book_name, \
        mm_metadata_book_json) \
        values ($1,$2,$3,$4,$5)",
    )
    .bind(new_guid)
    .bind(&json_data["data"][0]["isbn10"])
    .bind(&json_data["data"][0]["isbn13"])
    .bind(&json_data["data"][0]["title"])
    .bind(&json_data["data"][0])
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_book_guid_by_name(
    sqlx_pool: &sqlx::PgPool,
    book_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO can be more than one by name
    // TODO sort by release date
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_metadata_book_guid \
        from mm_metadata_book \
        where mm_metadata_book_name = $1",
    )
    .bind(book_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/*

// TODO port query
def db_meta_book_image_random(self, return_image_type='Cover'):
    """
    Find random book image
    """
    // TODO little bobby tables
    self.db_cursor.execute(
        'select mm_metadata_book_localimage_json->'Images'->>''
        + return_image_type + '' as image_json,mm_metadata_book_guid'
                              ' from mm_media,mm_metadata_book'
                              ' where mm_media_metadata_guid = mm_metadata_book_guid'
                              ' and (mm_metadata_book_localimage_json->'Images'->>''
        + return_image_type + ''' + ')::text != 'null''
                                     ' order by random() limit 1')
    try:
        // then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None

 */
