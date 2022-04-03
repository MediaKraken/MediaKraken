use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

/*

# TODO port query
async def db_iradio_insert(self, radio_channel, db_connection=None):
    """
    Insert iradio channel
    """
    # TODO exists
    if await db_conn.fetchval('select count(*) from mm_radio'
                              ' where mm_radio_address = $1',
                              radio_channel) == 0:
        new_guid = uuid.uuid4()
        self.db_cursor.execute('insert into mm_radio (mm_radio_guid,'
                               ' mm_radio_address,'
                               ' mm_radio_active)'
                               ' values ($1, $2, true)',
                               new_guid, radio_channel)
        return new_guid

# TODO port query
async def db_iradio_list(self, offset=0, records=None, active_station=True,
                         search_value=None, db_connection=None):
    """
    Iradio list
    """
    if search_value is not None:
        return await db_conn.fetch('select mm_radio_guid,'
                                   ' mm_radio_name,'
                                   ' mm_radio_address'
                                   ' from mm_radio where mm_radio_guid'
                                   ' in (select mm_radio_guid from mm_radio'
                                   ' where mm_radio_active = $1 and mm_radio_name % $2'
                                   ' order by LOWER(mm_radio_name) offset $3 limit $4)'
                                   ' order by LOWER(mm_radio_name)',
                                   active_station, search_value, offset, records)
    else:
        return await db_conn.fetch('select mm_radio_guid,'
                                   ' mm_radio_name,'
                                   ' mm_radio_address'
                                   ' from mm_radio where mm_radio_guid'
                                   ' in (select mm_radio_guid'
                                   ' from mm_radio'
                                   ' where mm_radio_active = $1'
                                   ' order by LOWER(mm_radio_name)'
                                   ' offset $2 limit $3)'
                                   ' order by LOWER(mm_radio_name)',
                                   active_station, offset, records)

# TODO port query
pub async fn mk_lib_database_media_iradio_count(pool: &sqlx::PgPool,
                                                  search_value: String)
                                                  -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

# TODO port query
async def db_iradio_list_count(self, active_station=True, search_value=None, db_connection=None):
    """
    Iradio count
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*) from mm_radio '
                                      'where mm_radio_active = $1'
                                      ' and mm_radio_name = $2',
                                      active_station)
    else:
        return await db_conn.fetchval('select count(*) from mm_radio'
                                      ' where mm_radio_active = $1',
                                      active_station)

 */