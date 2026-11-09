use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaTVLiveList {
    pub mm_tv_station_name: String,
    pub mm_tv_station_channel: String,
    pub mm_tv_schedule_json: serde_json::Value,
}

pub async fn mk_lib_database_meta_tv_live_read(
    sqlx_pool: &sqlx::PgPool,
    broadcast_time: DateTime<Utc>,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query(
        r#"select mm_tv_station_name, mm_tv_station_channel, mm_tv_schedule_json from mm_tv_stations, mm_tv_schedule where mm_tv_schedule_station_id = mm_tv_station_id and mm_tv_schedule_date = $1 order by LOWER(mm_tv_station_name), mm_tv_schedule_json->'airDateTime'"#,
    )
    .bind(broadcast_time)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct MetaTVStationList {
    pub mm_tv_stations_id: uuid::Uuid,
    mm_tv_station_name: String,
    mm_tv_station_id: String,
    mm_tv_station_channel: String,
}

pub async fn mk_lib_database_meta_tv_live_station_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<MetaTVStationList>, sqlx::Error> {
    let table_rows: Vec<MetaTVStationList> = sqlx::query_as(
        r#"select mm_tv_stations_id, mm_tv_station_name, mm_tv_station_id, mm_tv_station_channel from mm_tv_stations"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_meta_tv_station_exists(
    sqlx_pool: &sqlx::PgPool,
    station_id: String,
    channel_id: String,
) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(
        r#"select exists(select 1 from mm_tv_stations where mm_tv_station_id = $1 and mm_tv_station_channel = $2 limit 1) limit 1"#,
    )
    .bind(station_id)
    .bind(channel_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Insert a station/channel record unless it already exists; returns the new
/// guid when a row was inserted.
pub async fn mk_lib_database_meta_tv_station_insert(
    sqlx_pool: &sqlx::PgPool,
    station_id: &str,
    channel_id: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_tv_stations
        where mm_tv_station_id = $1 and mm_tv_station_channel = $2"#,
    )
    .bind(station_id)
    .bind(channel_id)
    .fetch_one(sqlx_pool)
    .await?;
    if row.0 > 0 {
        return Ok(None);
    }
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_tv_stations (mm_tv_stations_id, mm_tv_station_id,
        mm_tv_station_channel) values ($1, $2, $3)"#,
    )
    .bind(new_guid)
    .bind(station_id)
    .bind(channel_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(Some(new_guid))
}

/// Update station/channel info.
pub async fn mk_lib_database_meta_tv_station_update(
    sqlx_pool: &sqlx::PgPool,
    station_name: &str,
    station_id: &str,
    station_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_tv_stations
        set mm_tv_station_name = $1, mm_tv_station_json = $2
        where mm_tv_station_id = $3"#,
    )
    .bind(station_name)
    .bind(station_json)
    .bind(station_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Insert or update a day's schedule for a station; returns the new guid when
/// a row was inserted.
///
/// TODO change to upsert (the Python original did the same; there is no unique
/// constraint on station+date to conflict on).
pub async fn mk_lib_database_meta_tv_schedule_insert(
    sqlx_pool: &sqlx::PgPool,
    station_id: &str,
    schedule_date: chrono::NaiveDate,
    schedule_json: &serde_json::Value,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_tv_schedule
        where mm_tv_schedule_station_id = $1 and mm_tv_schedule_date = $2"#,
    )
    .bind(station_id)
    .bind(schedule_date)
    .fetch_one(&mut *transaction)
    .await?;
    if row.0 > 0 {
        sqlx::query(
            r#"update mm_tv_schedule set mm_tv_schedule_json = $1
            where mm_tv_schedule_station_id = $2
            and mm_tv_schedule_date = $3"#,
        )
        .bind(schedule_json)
        .bind(station_id)
        .bind(schedule_date)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(None)
    } else {
        let new_guid = uuid::Uuid::now_v7();
        sqlx::query(
            r#"insert into mm_tv_schedule (mm_tv_schedule_id,
            mm_tv_schedule_station_id, mm_tv_schedule_date, mm_tv_schedule_json)
            values ($1, $2, $3, $4)"#,
        )
        .bind(new_guid)
        .bind(station_id)
        .bind(schedule_date)
        .bind(schedule_json)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(Some(new_guid))
    }
}

/// Insert or update a program record; returns the new guid when a row was
/// inserted.
///
/// TODO change to upsert (the Python original did the same).
pub async fn mk_lib_database_meta_tv_program_insert(
    sqlx_pool: &sqlx::PgPool,
    program_id: &str,
    program_json: &serde_json::Value,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_tv_schedule_program
        where mm_tv_schedule_program_id = $1"#,
    )
    .bind(program_id)
    .fetch_one(&mut *transaction)
    .await?;
    if row.0 > 0 {
        sqlx::query(
            r#"update mm_tv_schedule_program
            set mm_tv_schedule_program_json = $1
            where mm_tv_schedule_program_id = $2"#,
        )
        .bind(program_json)
        .bind(program_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(None)
    } else {
        let new_guid = uuid::Uuid::now_v7();
        sqlx::query(
            r#"insert into mm_tv_schedule_program (mm_tv_schedule_program_guid,
            mm_tv_schedule_program_id, mm_tv_schedule_program_json)
            values ($1, $2, $3)"#,
        )
        .bind(new_guid)
        .bind(program_id)
        .bind(program_json)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(Some(new_guid))
    }
}
