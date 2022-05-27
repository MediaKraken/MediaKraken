#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_hardware_manufacturer_upsert(pool: &sqlx::PgPool,
                                                          manufacturer_name: String,
                                                          manufacturer_id: i32)
                                                          -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_hardware_manufacturer (mm_hardware_manu_guid, \
        mm_hardware_manu_name, mm_hardware_manu_gc_id) values ($1, $2, $3) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(uuid::Uuid::new_v4())
        .bind(manufacturer_name)
        .bind(manufacturer_id)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_type_upsert(pool: &sqlx::PgPool,
                                                  hardware_type: String)
                                                  -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_hardware_type (mm_hardware_type_guid, \
        mm_hardware_type_name) values ($1, $2) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(uuid::Uuid::new_v4())
        .bind(hardware_type)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_device_count(pool: &sqlx::PgPool,
                                                   hardware_manufacturer: String,
                                                   mm_hardware_model: String)
                                                   -> Result<i32, sqlx::Error> {
    if mm_hardware_model != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_hardware \
            where mm_hardware_manufacturer = $1 and mm_hardware_model = $2")
            .bind(hardware_manufacturer)
            .bind(mm_hardware_model)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_hardware \
            where mm_hardware_manufacturer = $1")
            .bind(hardware_manufacturer)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_hardware_json_read(pool: &sqlx::PgPool,
                                                manufacturer: String,
                                                model_name: String)
                                                -> Result<(serde_json::Value), sqlx::Error> {
    let row: (serde_json::Value, ) = sqlx::query_as("select mm_hardware_json \
        from mm_hardware_json where mm_hardware_manufacturer = $1 and mm_hardware_model = $2")
        .bind(manufacturer)
        .bind(model_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_hardware_insert(pool: &sqlx::PgPool,
                                             manufacturer: String,
                                             model_name: String,
                                             json_data: serde_json::Value)
                                             -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_hardware_json(mm_hardware_id, mm_hardware_manufacturer, \
        mm_hardware_model, mm_hardware_json) values($1, $2, $3, $4)")
        .bind(new_guid)
        .bind(manufacturer)
        .bind(model_name)
        .bind(json_data)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_hardware_delete(pool: &sqlx::PgPool,
                                             hardware_uuid: Uuid)
                                             -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_hardware_json where mm_hardware_id = $1")
        .bind(hardware_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}