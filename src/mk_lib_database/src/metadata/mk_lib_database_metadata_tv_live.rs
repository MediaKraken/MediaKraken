use uuid::Uuid;
use sqlx::postgres::PgRow;


pub async fn mk_lib_database_meta_tv_live_read(pool: &sqlx::PgPool,
                                               broadcast_time: chrono::DateTime)
                                               -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_tv_station_name, mm_tv_station_channel, \
        mm_tv_schedule_json from mm_tv_stations, mm_tv_schedule \
        where mm_tv_schedule_station_id = mm_tv_station_id  and mm_tv_schedule_date = $1 \
        order by LOWER(mm_tv_station_name), mm_tv_schedule_json->'airDateTime'")
        .bind(broadcast_time)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
