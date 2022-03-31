use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_media_game_clone_read(pool: &sqlx::PgPool)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select gi_id, gi_cloneof from mm_game_info \
        where gi_system_id is null and gi_cloneof IS NOT NULL and gi_gc_category is null")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_media_category_by_name(pool: &sqlx::PgPool,
                                                    category_name: String)
                                                    -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query("select gi_gc_category from mm_game_info \
        where gi_short_name = $1")
        .bind(category_name)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

pub async fn mk_lib_database_media_game_category_update(pool: &sqlx::PgPool,
                                                        category: String,
                                                        game_id: uuid::Uuid)
                                                        -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_game_info set gi_gc_category = $1 where gi_id = $2")
        .bind(category)
        .bind(game_id)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}