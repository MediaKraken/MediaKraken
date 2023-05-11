use mk_lib_logging::mk_lib_logging;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_game_server_delete(
    sqlx_pool: &sqlx::PgPool,
    game_server_uuid: Uuid,
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
    sqlx::query("delete from mm_game_dedicated_servers where mm_game_server_guid = $1")
        .bind(game_server_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBGameServerList {
    pub mm_game_server_guid: uuid::Uuid,
    pub mm_game_server_name: String,
    pub mm_game_server_json: serde_json::Value,
}

pub async fn mk_lib_database_game_server_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBGameServerList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_game_server_guid, mm_game_server_name, \
            mm_game_server_json from mm_game_dedicated_servers \
            where mm_game_server_name = $1 \
            order by mm_game_server_name offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_game_server_guid, mm_game_server_name, \
            mm_game_server_json from mm_game_dedicated_servers \
            order by mm_game_server_name offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBGameServerList> = select_query
        .map(|row: PgRow| DBGameServerList {
            mm_game_server_guid: row.get("mm_game_server_guid"),
            mm_game_server_name: row.get("mm_game_server_name"),
            mm_game_server_json: row.get("mm_game_server_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_game_server_detail(
    sqlx_pool: &sqlx::PgPool,
    game_server_uuid: Uuid,
) -> Result<PgRow, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: PgRow = sqlx::query(
        "select mm_game_server_name, mm_game_server_json \
        from mm_game_dedicated_servers where mm_game_server_guid = $1",
    )
    .bind(game_server_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_game_server_count(
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
            "select count(*) from mm_game_dedicated_servers \
            where mm_game_server_name = $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_game_dedicated_servers")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_game_server_upsert(
    sqlx_pool: &sqlx::PgPool,
    server_name: String,
    server_json: serde_json::Value,
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
    // TODO um, would return "invalid" uuid on update
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "INSERT INTO mm_game_dedicated_servers(mm_game_server_guid, \
        mm_game_server_name, mm_game_server_json) VALUES($ 1, $2, $3) \
        ON CONFLICT(mm_game_server_name) DO UPDATE SET mm_game_server_json = $ 4",
    )
    .bind(new_guid)
    .bind(server_name)
    .bind(&server_json)
    .bind(&server_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
