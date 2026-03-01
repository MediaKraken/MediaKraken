use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_media_game_clone_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query(
        r#"select gi_id, gi_cloneof from mm_metadata_game_software_info
        where gi_system_id is null and gi_cloneof IS NOT NULL and gi_gc_category is null"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

pub async fn mk_lib_database_media_category_by_name(
    sqlx_pool: &sqlx::PgPool,
    category_name: String,
) -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"select gi_gc_category from mm_metadata_game_software_info
        where gi_short_name = $1"#,
    )
    .bind(category_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_media_game_category_update(
    sqlx_pool: &sqlx::PgPool,
    category: String,
    game_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_game_software_info set gi_gc_category = $1 where gi_id = $2"#,
    )
    .bind(category)
    .bind(game_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaGameList {
    pub mm_metadata_game_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_game_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaGameList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as("")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as("")
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

pub async fn mk_lib_database_media_game_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as("")
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_game_software_info"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
