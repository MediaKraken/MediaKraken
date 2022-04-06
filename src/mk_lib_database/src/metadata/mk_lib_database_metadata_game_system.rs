use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_metadata_game_system_by_uuid(pool: &sqlx::PgPool,
                                                          game_sys_uuid: Uuid)
                                                          -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query("select * from mm_metadata_game_systems_info \
        where gs_id = $1")
        .bind(game_sys_uuid)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_game_system_count(pool: &sqlx::PgPool,
                                                        search_value: String)
                                                        -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_game_systems_info \
            where gs_game_system_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_game_systems_info")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameSystemList {
    gs_id: uuid::Uuid,
    gs_game_system_name: String,
    gs_description: String,
    gs_year: String,
    gs_game_system_alias: String,
}

pub async fn mk_lib_database_metadata_game_system_read(pool: &sqlx::PgPool,
                                                       search_value: String,
                                                       offset: i32, limit: i32)
                                                       -> Result<Vec<DBMetaGameSystemList>, sqlx::Error> {
    // TODO might need to sort by release year as well for machines with multiple releases
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select gs_id, gs_game_system_name, \
            gs_game_system_json->'description' as gs_description, \
            gs_game_system_json->'year' as gs_year, \
            gs_game_system_alias from mm_metadata_game_systems_info \
            where gs_game_system_name % $1 \
            order by gs_game_system_json->'description' \
            offset $2 limit $2")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select gs_id,gs_game_system_name, \
            gs_game_system_json->'description' as gs_description, \
            gs_game_system_json->'year' as gs_year, \
            gs_game_system_alias from mm_metadata_game_systems_info \
            order by gs_game_system_json->'description' offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMetaGameSystemList> = select_query
        .map(|row: PgRow| DBMetaGameSystemList {
            gs_id: row.get("gs_id"),
            gs_game_system_name: row.get("gs_game_system_name"),
            gs_description: row.get("gs_description"),
            gs_year: row.get("gs_year"),
            gs_game_system_alias: row.get("gs_game_system_alias"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_game_system_upsert(pool: &sqlx::PgPool,
                                                         system_name: String,
                                                         system_alias: String,
                                                         system_json: serde_json::Value)
                                                         -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("INSERT INTO mm_metadata_game_systems_info \
        (gs_game_system_id, \
        gs_game_system_name, \
        gs_game_system_alias, \
        gs_game_system_json) \
        VALUES ($1, $2, $3, $4) \
        ON CONFLICT (gs_game_system_name) \
        DO UPDATE SET gs_game_system_alias = $5, gs_game_system_json = $6")
        .bind(new_guid)
        .bind(system_name)
        .bind(system_alias)
        .bind(system_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_game_system_guid_by_short_name(pool: &sqlx::PgPool,
                                                                     game_system_short_name: String)
                                                                     -> Result<Uuid, sqlx::Error> {
    let row: (Uuid, ) = sqlx::query_as("select gs_game_system_id \
        from mm_metadata_game_systems_info \
        where gs_game_system_name = $1")
        .bind(game_system_short_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_system_game_count_by_short_name(pool: &sqlx::PgPool,
                                                                           game_system_short_name: String)
                                                                           -> Result<i32, sqlx::Error> {
    // TODO this query doesn't return game count.......
    let row: (i32, ) = sqlx::query_as("select count(*) \
        from mm_metadata_game_systems_info \
        where gs_game_system_name = $1")
        .bind(game_system_short_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
