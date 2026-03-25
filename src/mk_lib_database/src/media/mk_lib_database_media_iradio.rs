use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaIradioList {
    pub mm_radio_guid: uuid::Uuid,
    pub mm_radio_name: Option<String>,
    pub mm_radio_address: String,
}

pub async fn mk_lib_database_media_iradio_insert(
    sqlx_pool: &sqlx::PgPool,
    radio_channel: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v7();
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"insert into mm_radio (
            mm_radio_guid,
            mm_radio_address,
            mm_radio_active
        ) values ($1, $2, true)
        on conflict (mm_radio_address) do nothing
        returning mm_radio_guid"#,
    )
    .bind(new_guid)
    .bind(radio_channel)
    .fetch_optional(sqlx_pool)
    .await?;

    Ok(row.map(|(radio_guid,)| radio_guid))
}

pub async fn mk_lib_database_media_iradio_upsert(
    sqlx_pool: &sqlx::PgPool,
    station_uuid: &str,
    station_name: &str,
    station_address: &str,
    station_country: Option<&str>,
    station_language: Option<&str>,
    station_tags: Option<&str>,
    station_url_resolved: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"insert into mm_radio (
            mm_radio_guid,
            mm_radio_stationuuid,
            mm_radio_name,
            mm_radio_address,
            mm_radio_country,
            mm_radio_language,
            mm_radio_tags,
            mm_radio_url_resolved,
            mm_radio_active
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, true)
        on conflict (mm_radio_address) do update set
            mm_radio_stationuuid = excluded.mm_radio_stationuuid,
            mm_radio_name = excluded.mm_radio_name,
            mm_radio_country = excluded.mm_radio_country,
            mm_radio_language = excluded.mm_radio_language,
            mm_radio_tags = excluded.mm_radio_tags,
            mm_radio_url_resolved = excluded.mm_radio_url_resolved,
            mm_radio_active = true
        returning mm_radio_guid"#,
    )
    .bind(uuid::Uuid::new_v7())
    .bind(station_uuid)
    .bind(station_name)
    .bind(station_address)
    .bind(station_country)
    .bind(station_language)
    .bind(station_tags)
    .bind(station_url_resolved)
    .fetch_one(sqlx_pool)
    .await?;

    Ok(row.0)
}

pub async fn mk_lib_database_media_iradio_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    records: Option<i64>,
    active_station: bool,
    search_value: Option<String>,
) -> Result<Vec<DBMediaIradioList>, sqlx::Error> {
    match (search_value, records) {
        (Some(search), Some(limit)) => {
            sqlx::query_as(
                r#"select mm_radio_guid, mm_radio_name, mm_radio_address
                from mm_radio where mm_radio_guid
                in (select mm_radio_guid from mm_radio
                where mm_radio_active = $1 and mm_radio_name % $2
                order by LOWER(mm_radio_name) offset $3 limit $4)
                order by LOWER(mm_radio_name)"#,
            )
            .bind(active_station)
            .bind(search)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
        }
        (Some(search), None) => {
            sqlx::query_as(
                r#"select mm_radio_guid, mm_radio_name, mm_radio_address
                from mm_radio where mm_radio_guid
                in (select mm_radio_guid from mm_radio
                where mm_radio_active = $1 and mm_radio_name % $2
                order by LOWER(mm_radio_name) offset $3)
                order by LOWER(mm_radio_name)"#,
            )
            .bind(active_station)
            .bind(search)
            .bind(offset)
            .fetch_all(sqlx_pool)
            .await
        }
        (None, Some(limit)) => {
            sqlx::query_as(
                r#"select mm_radio_guid, mm_radio_name, mm_radio_address
                from mm_radio where mm_radio_guid
                in (select mm_radio_guid
                from mm_radio
                where mm_radio_active = $1
                order by LOWER(mm_radio_name)
                offset $2 limit $3)
                order by LOWER(mm_radio_name)"#,
            )
            .bind(active_station)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
        }
        (None, None) => {
            sqlx::query_as(
                r#"select mm_radio_guid, mm_radio_name, mm_radio_address
                from mm_radio where mm_radio_guid
                in (select mm_radio_guid
                from mm_radio
                where mm_radio_active = $1
                order by LOWER(mm_radio_name)
                offset $2)
                order by LOWER(mm_radio_name)"#,
            )
            .bind(active_station)
            .bind(offset)
            .fetch_all(sqlx_pool)
            .await
        }
    }
}

pub async fn mk_lib_database_media_iradio_count(
    sqlx_pool: &sqlx::PgPool,
    active_station: bool,
    search_value: Option<String>,
) -> Result<i64, sqlx::Error> {
    if let Some(search) = search_value {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_radio
            where mm_radio_active = $1 and mm_radio_name = $2"#,
        )
        .bind(active_station)
        .bind(search)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_radio
            where mm_radio_active = $1"#,
        )
        .bind(active_station)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
