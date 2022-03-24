use uuid::Uuid;
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

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

/*
def db_tv_stations_read(self):
    """
    # read the stations
    """
    self.db_cursor.execute('select mm_tv_stations_id'
                           ',mm_tv_station_name,'
                           'mm_tv_station_id,'
                           'mm_tv_station_channel'
                           ' from mm_tv_stations')
    return self.db_cursor.fetchall()


def db_tv_stations_read_stationid_list(self):
    """
    # read the stationid list
    """
    self.db_cursor.execute('select mm_tv_station_id'
                           ' from mm_tv_stations')
    return self.db_cursor.fetchall()


def db_tv_station_insert(self, station_id, channel_id):
    """
    # insert station/channel unless it exists
    """
    if self.db_tv_station_exist(station_id, channel_id) == 0:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_stations (mm_tv_stations_id,'
                               ' mm_tv_station_id,'
                               ' mm_tv_station_channel)'
                               ' values (%s, %s, %s)',
                               (new_guid, station_id, channel_id))
        self.db_commit()
        return new_guid


def db_tv_station_exist(self, station_id, channel_id):
    """
    # channel exist check
    """
    self.db_cursor.execute('select exists(select 1 from mm_tv_stations'
                           ' where mm_tv_station_id = %s'
                           ' and mm_tv_station_channel = %s limit 1) limit 1',
                           (station_id, channel_id))
    return self.db_cursor.fetchone()[0]


def db_tv_station_update(self, station_name, station_id, station_json):
    """
    # update station/channel info
    """
    self.db_cursor.execute('update mm_tv_stations set mm_tv_station_name = %s,'
                           ' mm_tv_station_json = %s'
                           ' where mm_tv_station_id = %s',
                           (station_name, station_json, station_id))
    self.db_commit()


def db_tv_schedule_insert(self, station_id, schedule_date, schedule_json):
    """
    # insert schedule info
    """
    self.db_cursor.execute('select count(*) from mm_tv_schedule'
                           ' where mm_tv_schedule_station_id = %s and mm_tv_schedule_date = %s',
                           (station_id, schedule_date))
    # TODO change to upsert
    if self.db_cursor.fetchone()[0] > 0:
        self.db_cursor.execute('update mm_tv_schedule set mm_tv_schedule_json = %s'
                               ' where mm_tv_schedule_station_id = %s'
                               ' and mm_tv_schedule_date = %s',
                               (schedule_json, station_id, schedule_date))
        self.db_commit()
    else:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_schedule (mm_tv_schedule_id,'
                               ' mm_tv_schedule_station_id,'
                               ' mm_tv_schedule_date,'
                               ' mm_tv_schedule_json)'
                               ' values (%s, %s, %s, %s)',
                               (new_guid, station_id, schedule_date, schedule_json))
        self.db_commit()
        return new_guid


def db_tv_program_insert(self, program_id, program_json):
    """
    # insert program info
    """
    self.db_cursor.execute('select count(*) from mm_tv_schedule_program'
                           ' where mm_tv_schedule_program_id = %s', (program_id,))
    # TODO change to upsert
    if self.db_cursor.fetchone()[0] > 0:
        self.db_cursor.execute('update mm_tv_schedule_program'
                               ' set mm_tv_schedule_program_json = %s'
                               ' where mm_tv_schedule_program_id = %s',
                               (program_json, program_id))
        self.db_commit()
    else:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_tv_schedule_program'
                               ' (mm_tv_schedule_program_guid,'
                               ' mm_tv_schedule_program_id,'
                               ' mm_tv_schedule_program_json) values (%s, %s, %s)',
                               (new_guid, program_id, program_json))
        self.db_commit()
        return new_guid


def db_tv_schedule_by_date(self, display_date):
    """
    # tv shows for schedule display
    """
    self.db_cursor.execute('select mm_tv_station_name,'
                           ' mm_tv_station_channel,'
                           ' mm_tv_schedule_json'
                           ' from mm_tv_stations, mm_tv_schedule'
                           ' where mm_tv_schedule_station_id = mm_tv_station_id'
                           ' and mm_tv_schedule_date = %s'
                           ' order by LOWER(mm_tv_station_name),'
                           ' mm_tv_schedule_json->\'airDateTime\'',
                           (display_date,))
    return self.db_cursor.fetchall()
 */