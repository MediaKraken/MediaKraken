use sqlx::postgres::PgRow;
use uuid::Uuid;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_game_server_delete(pool: &sqlx::PgPool,
                                                game_server_uuid: uuid::Uuid)
                                                -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_game_dedicated_servers where mm_game_server_guid = $1")
        .bind(game_server_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBGameServerList {
	mm_game_server_guid: uuid::Uuid,
	mm_game_server_name: String,
    mm_game_server_json: Json,
}

pub async fn mk_lib_database_dedicated_server_read(pool: &sqlx::PgPool,
                                                   offset: i32, limit: i32)
                                                   -> Result<Vec<DBGameServerList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_game_server_guid, mm_game_server_name, \
        mm_game_server_json from mm_game_dedicated_servers \
        order by mm_game_server_name offset $1 limit $2")
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBGameServerList> = select_query
		.map(|row: PgRow| DBGameServerList {
			mm_game_server_guid: row.get("mm_game_server_guid"),
			mm_game_server_name: row.get("mm_game_server_name"),
			mm_game_server_json: row.get("mm_game_server_json"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}


pub async fn mk_lib_database_game_server_detail(pool: &sqlx::PgPool,
                                                game_server_uuid: uuid::Uuid)
                                                -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_game_server_name, mm_game_server_json \
        from mm_game_dedicated_servers where mm_game_server_guid = $1")
        .bind(game_server_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}


pub async fn mk_lib_database_game_server_list_count(pool: &sqlx::PgPool,
                                                    search_value: String)
                                                    -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_game_dedicated_servers \
            where mm_game_server_name = $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_game_dedicated_servers")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}


pub async fn mk_lib_database_game_server_upsert(pool: &sqlx::PgPool,
                                                server_name: String,
                                                server_json: serde_json::Value)
                                                -> Result<(uuid::Uuid), sqlx::Error> {
    // TODO um, would return "invalid" uuid on update
    new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query_as("INSERT INTO mm_game_dedicated_servers(mm_game_server_guid, \
        mm_game_server_name, mm_game_server_json) VALUES($ 1, $2, $3) \
        ON CONFLICT(mm_game_server_name) DO UPDATE SET mm_game_server_json = $ 4")
        .bind(new_guid)
        .bind(server_name)
        .bind(server_json)
        .bind(server_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
