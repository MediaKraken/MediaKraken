use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_hardware_manufacturer_upsert(
    sqlx_pool: &sqlx::PgPool,
    manufacturer_name: String,
    manufacturer_id: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_hardware_manufacturer (mm_hardware_manu_guid, \
        mm_hardware_manu_name, mm_hardware_manu_gc_id) values ($1, $2, $3) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(manufacturer_name)
    .bind(manufacturer_id)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_type_upsert(
    sqlx_pool: &sqlx::PgPool,
    hardware_type: String,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_hardware_type (mm_hardware_type_guid, \
        mm_hardware_type_name) values ($1, $2) \
        ON CONFLICT (mm_hardware_type_name) DO NOTHING",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(hardware_type)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_model_insert(
    sqlx_pool: &sqlx::PgPool,
    hardware_manufacturer: String,
    hardware_type: String,
    hardware_model: String,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_hardware_model (mm_hardware_model_guid, \
        mm_hardware_manufacturer, \
        mm_hardware_model_type, \
        mm_hardware_model_name) values ($1, $2, $3, $4)",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(hardware_manufacturer)
    .bind(hardware_type)
    .bind(hardware_model)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_model_device_count_by_type(
    sqlx_pool: &sqlx::PgPool,
    hardware_manufacturer: String,
    hardware_type: String,
    hardware_model: String,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_hardware_model \
            where mm_hardware_manufacturer = $1 and mm_hardware_model_type = $2 and mm_hardware_model_name = $3",
        )
        .bind(hardware_manufacturer)
        .bind(hardware_type)
        .bind(hardware_model)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_hardware_json_read_by_type(
    sqlx_pool: &sqlx::PgPool,
    manufacturer: String,
    model_name: String,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        "select mm_hardware_json \
        from mm_hardware_json where mm_hardware_manufacturer = $1 and mm_hardware_model = $2",
    )
    .bind(manufacturer)
    .bind(model_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_hardware_device_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("select count(*) from mm_hardware_json")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBDeviceList {
    pub mm_hardware_manufacturer: String,
    pub mm_hardware_model_type: String,
    pub mm_hardware_model_name: String,
}

pub async fn mk_lib_database_hardware_device_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBDeviceList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_hardware_manufacturer, \
            mm_hardware_model_type, \
            mm_hardware_model_name \
            from mm_hardware_model order by mm_hardware_manufacturer, \
            mm_hardware_model_type, mm_hardware_model_name desc",
    );
    let table_rows: Vec<DBDeviceList> = select_query
        .map(|row: PgRow| DBDeviceList {
            mm_hardware_manufacturer: row.get("mm_hardware_manufacturer"),
            mm_hardware_model_type: row.get("mm_hardware_model_type"),
            mm_hardware_model_name: row.get("mm_hardware_model_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_hardware_insert(
    sqlx_pool: &sqlx::PgPool,
    manufacturer: String,
    model_name: String,
    json_data: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_hardware_json(mm_hardware_id, mm_hardware_manufacturer, \
        mm_hardware_model, mm_hardware_json) values($1, $2, $3, $4)",
    )
    .bind(new_guid)
    .bind(manufacturer)
    .bind(model_name)
    .bind(json_data)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_hardware_delete(
    sqlx_pool: &sqlx::PgPool,
    hardware_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_hardware_json where mm_hardware_id = $1")
        .bind(hardware_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
