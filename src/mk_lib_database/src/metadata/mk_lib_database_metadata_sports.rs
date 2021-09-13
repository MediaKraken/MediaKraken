use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_metadata_sports_count(pool: &sqlx::PgPool,
                                                   search_value: String)
                                                   -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_sports \
            where mm_metadata_sports_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_sports")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_sports_read(pool: &sqlx::PgPool,
                                                 search_value: String,
                                                 offset: i32, limit: i32)
                                                 -> Result<Vec<PgRow>, sqlx::Error> {
    // TODO order by year
    if search_value != "" {
        let rows = sqlx::query("select mm_metadata_sports_guid, mm_metadata_sports_name \
            from mm_metadata_sports where mm_metadata_sports_guid \
            in (select mm_metadata_sports_guid from mm_metadata_sports \
            where mm_metadata_sports_name % $1 \
            order by LOWER(mm_metadata_sports_name) \
            offset $2 limit $3) \
            order by LOWER(mm_metadata_sports_name)")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select mm_metadata_sports_guid, mm_metadata_sports_name \
            from mm_metadata_sports where mm_metadata_sports_guid \
            in (select mm_metadata_sports_guid from mm_metadata_sports \
            order by LOWER(mm_metadata_sports_name) \
            offset $1 limit $2) \
            order by LOWER(mm_metadata_sports_name)")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

/*

async def db_meta_sports_guid_by_thesportsdb(self, thesports_uuid, db_connection=None):
    """
    # metadata guid by thesportsdb id
    """
    return await db_conn.fetchval('select mm_metadata_sports_guid'
                                  ' from mm_metadata_sports'
                                  ' where mm_metadata_media_sports_id->\'thesportsdb\''
                                  ' ? $1',
                                  thesports_uuid)


def db_meta_sports_guid_by_event_name(self, event_name):
    """
    # fetch guid by event name
    """
    self.db_cursor.execute('select mm_metadata_sports_guid'
                           ' from mm_metadata_sports'
                           ' where mm_metadata_sports_name = %s', (event_name,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_sports_guid']
    except:
        return None



def db_metathesportsdb_select_guid(self, guid):
    """
    # select
    """
    self.db_cursor.execute('select mm_metadata_sports_json'
                           ' from mm_metadata_sports'
                           ' where mm_metadata_sports_guid = %s', (guid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_sports_json']
    except:
        return None


def db_metathesportsdb_insert(self, series_id_json, event_name, show_detail,
                              image_json):
    """
    # insert
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_sports (mm_metadata_sports_guid,'
                           ' mm_metadata_media_sports_id,'
                           ' mm_metadata_sports_name,'
                           ' mm_metadata_sports_json,'
                           ' mm_metadata_sports_image_json)'
                           ' values (%s,%s,%s,%s,%s)',
                           (new_guid, series_id_json, event_name, show_detail, image_json))
    self.db_commit()
    return new_guid


def db_metathesports_update(self, series_id_json, event_name, show_detail,
                            sportsdb_id):
    """
    # updated
    """
    self.db_cursor.execute('update mm_metadata_sports'
                           ' set mm_metadata_media_sports_id = %s,'
                           ' mm_metadata_sports_name = %s,'
                           ' mm_metadata_sports_json = %s'
                           ' where mm_metadata_media_sports_id->\'thesportsdb\' ? %s',
                           (series_id_json, event_name, show_detail, sportsdb_id))
    self.db_commit()

 */