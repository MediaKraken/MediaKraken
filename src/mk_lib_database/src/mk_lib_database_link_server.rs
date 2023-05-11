use mk_lib_logging::mk_lib_logging;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_link_delete(
    sqlx_pool: &sqlx::PgPool,
    link_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_link where mm_link_guid = $1")
        .bind(link_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLinkList {
    pub mm_link_guid: uuid::Uuid,
    pub mm_link_name: String,
    pub mm_link_json: serde_json::Value,
}

pub async fn mk_lib_database_link_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    records: i64,
) -> Result<Vec<DBLinkList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query = sqlx::query(
        "select mm_link_guid, mm_link_name, \
        mm_link_json from mm_link \
        order by mm_link_name \
        offset $1 limit $2",
    )
    .bind(offset)
    .bind(records);
    let table_rows: Vec<DBLinkList> = select_query
        .map(|row: PgRow| DBLinkList {
            mm_link_guid: row.get("mm_link_guid"),
            mm_link_name: row.get("mm_link_name"),
            mm_link_json: row.get("mm_link_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_link_insert(
    sqlx_pool: &sqlx::PgPool,
    link_json: serde_json::Value,
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
    let new_guid = Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_link (mm_link_guid, mm_link_json) \
        values ($1, $2)",
    )
    .bind(new_guid)
    .bind(link_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_link_list_count(
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
            "select count(*) from mm_library_link \
            where mm_link_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_library_link")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
