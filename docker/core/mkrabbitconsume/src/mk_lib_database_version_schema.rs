#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde_json::{json, Value};
use sqlx::{types::Json, types::Uuid};

#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;

pub async fn mk_lib_database_update_schema(
    pool: &sqlx::PgPool,
    version_no: i32,
) -> Result<bool, sqlx::Error> {
    if version_no < 44 {
        // set mame version to 240
        let option_json: Value = mk_lib_database_option_status::mk_lib_database_option_read(&pool)
            .await
            .unwrap();
        // option_json["MAME"]["Version"] = 240;
        // mk_lib_database_option_status::mk_lib_database_option_update(&pool, option_json).await?;
        mk_lib_database_version_update(&pool, 44).await?;
    }
    if version_no < 45 {
        let mut transaction = pool.begin().await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_software_info \
            RENAME COLUMN gi_id TO gi_game_info_id;",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_software_info \
            RENAME COLUMN gi_system_id TO gi_game_info_system_id;",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&pool, 45).await?;
    }
    if version_no < 46 {
        let mut transaction = pool.begin().await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_systems_info \
            RENAME COLUMN gs_id TO gs_game_system_id;",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "INSERT INTO mm_metadata_game_systems_info \
            (gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias) \
            VALUES ($1, 'Arcade', 'Arcade')",
        )
        .bind(uuid::Uuid::nil())
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&pool, 46).await?;
    }
    Ok(true)
}

pub async fn mk_lib_database_version_update(
    pool: &sqlx::PgPool,
    version_number: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_version set mm_version_number = $1")
        .bind(version_number)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
