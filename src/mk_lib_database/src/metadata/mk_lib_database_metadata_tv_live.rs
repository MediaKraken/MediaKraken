use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

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
        "select mm_tv_station_name, mm_tv_station_channel, \
        mm_tv_schedule_json from mm_tv_stations, mm_tv_schedule \
        where mm_tv_schedule_station_id = mm_tv_station_id and mm_tv_schedule_date = $1 \
        order by LOWER(mm_tv_station_name), mm_tv_schedule_json->'airDateTime'",
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
    let select_query = sqlx::query(
        "select mm_tv_stations_id, mm_tv_station_name, \
        mm_tv_station_id, mm_tv_station_channel \
        from mm_tv_stations",
    );
    let table_rows: Vec<MetaTVStationList> = select_query
        .map(|row: PgRow| MetaTVStationList {
            mm_tv_stations_id: row.get("mm_tv_stations_id"),
            mm_tv_station_name: row.get("mm_tv_station_name"),
            mm_tv_station_id: row.get("mm_tv_station_id"),
            mm_tv_station_channel: row.get("mm_tv_station_channel"),
        })
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
        "select exists(select 1 from mm_tv_stations \
        where mm_tv_station_id = $1 \
        and mm_tv_station_channel = $2 limit 1) limit 1",
    )
    .bind(station_id)
    .bind(channel_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/*
// TODO port query
def db_tv_station_insert(self, station_id, channel_id):
    """
    # insert station/channel unless it exists
    """
    if self.db_tv_station_exist(station_id, channel_id) == 0:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_stations (mm_tv_stations_id,'
                               ' mm_tv_station_id,'
                               ' mm_tv_station_channel)'
                               ' values ($1, $2, $3)',
                               (new_guid, station_id, channel_id))
        self.db_commit()
        return new_guid

// TODO port query
def db_tv_station_update(self, station_name, station_id, station_json):
    """
    # update station/channel info
    """
    self.db_cursor.execute('update mm_tv_stations set mm_tv_station_name = $1,'
                           ' mm_tv_station_json = $2'
                           ' where mm_tv_station_id = $3',
                           (station_name, station_json, station_id))
    self.db_commit()


// TODO port query
def db_tv_schedule_insert(self, station_id, schedule_date, schedule_json):
    """
    # insert schedule info
    """
    self.db_cursor.execute('select count(*) from mm_tv_schedule'
                           ' where mm_tv_schedule_station_id = $1 and mm_tv_schedule_date = $2',
                           (station_id, schedule_date))
    // TODO change to upsert
    if self.db_cursor.fetchone()[0] > 0:
        self.db_cursor.execute('update mm_tv_schedule set mm_tv_schedule_json = $1'
                               ' where mm_tv_schedule_station_id = $2'
                               ' and mm_tv_schedule_date = $3',
                               (schedule_json, station_id, schedule_date))
        self.db_commit()
    else:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_schedule (mm_tv_schedule_id,'
                               ' mm_tv_schedule_station_id,'
                               ' mm_tv_schedule_date,'
                               ' mm_tv_schedule_json)'
                               ' values ($1, $2, $3, $4)',
                               (new_guid, station_id, schedule_date, schedule_json))
        self.db_commit()
        return new_guid


// TODO port query
def db_tv_program_insert(self, program_id, program_json):
    """
    # insert program info
    """
    self.db_cursor.execute('select count(*) from mm_tv_schedule_program'
                           ' where mm_tv_schedule_program_id = $1', (program_id,))
    // TODO change to upsert
    if self.db_cursor.fetchone()[0] > 0:
        self.db_cursor.execute('update mm_tv_schedule_program'
                               ' set mm_tv_schedule_program_json = $1'
                               ' where mm_tv_schedule_program_id = $2',
                               (program_json, program_id))
        self.db_commit()
    else:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_schedule_program'
                               ' (mm_tv_schedule_program_guid,'
                               ' mm_tv_schedule_program_id,'
                               ' mm_tv_schedule_program_json) values ($1, $2, $3)',
                               (new_guid, program_id, program_json))
        self.db_commit()
        return new_guid


// TODO port query
def db_tv_schedule_by_date(self, display_date):
    """
    # tv shows for schedule display
    """
    self.db_cursor.execute('select mm_tv_station_name,'
                           ' mm_tv_station_channel,'
                           ' mm_tv_schedule_json'
                           ' from mm_tv_stations, mm_tv_schedule'
                           ' where mm_tv_schedule_station_id = mm_tv_station_id'
                           ' and mm_tv_schedule_date = $1'
                           ' order by LOWER(mm_tv_station_name),'
                           ' mm_tv_schedule_json->'airDateTime'',
                           (display_date,))
    return self.db_cursor.fetchall()
 */
