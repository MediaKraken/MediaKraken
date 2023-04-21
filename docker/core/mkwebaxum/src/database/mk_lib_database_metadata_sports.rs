#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_metadata_sports_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_sports \
            where mm_metadata_sports_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_sports")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaSportsList {
    pub mm_metadata_sports_guid: uuid::Uuid,
    pub mm_metadata_sports_name: String,
}

pub async fn mk_lib_database_metadata_sports_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaSportsList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO order by year
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_sports_guid, mm_metadata_sports_name \
            from mm_metadata_sports where mm_metadata_sports_guid \
            where mm_metadata_sports_name % $1 \
            order by LOWER(mm_metadata_sports_name) \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_sports_guid, mm_metadata_sports_name \
            from mm_metadata_sports
            order by LOWER(mm_metadata_sports_name) \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaSportsList> = select_query
        .map(|row: PgRow| DBMetaSportsList {
            mm_metadata_sports_guid: row.get("mm_metadata_sports_guid"),
            mm_metadata_sports_name: row.get("mm_metadata_sports_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

/*

// TODO port query
pub async fn db_meta_sports_guid_by_thesportsdb(self, thesports_uuid):
    """
    # metadata guid by thesportsdb id
    """
    return await db_conn.fetchval('select mm_metadata_sports_guid'
                                  ' from mm_metadata_sports'
                                  ' where mm_metadata_media_sports_id->'thesportsdb''
                                  ' ? $1',
                                  thesports_uuid)


// TODO port query
def db_meta_sports_guid_by_event_name(self, event_name):
    """
    # fetch guid by event name
    """
    self.db_cursor.execute('select mm_metadata_sports_guid'
                           ' from mm_metadata_sports'
                           ' where mm_metadata_sports_name = $1', (event_name,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_sports_guid']
    except:
        return None



// TODO port query
def db_metathesportsdb_select_guid(self, guid):
    """
    # select
    """
    self.db_cursor.execute('select mm_metadata_sports_json'
                           ' from mm_metadata_sports'
                           ' where mm_metadata_sports_guid = $1', (guid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_sports_json']
    except:
        return None


// TODO port query
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
                           ' values ($1,$2,$3,$4,$5)',
                           (new_guid, series_id_json, event_name, show_detail, image_json))
    self.db_commit()
    return new_guid


// TODO port query
def db_metathesports_update(self, series_id_json, event_name, show_detail,
                            sportsdb_id):
    """
    # updated
    """
    self.db_cursor.execute('update mm_metadata_sports'
                           ' set mm_metadata_media_sports_id = $1,'
                           ' mm_metadata_sports_name = $2,'
                           ' mm_metadata_sports_json = $3'
                           ' where mm_metadata_media_sports_id->'thesportsdb' ? $4',
                           (series_id_json, event_name, show_detail, sportsdb_id))
    self.db_commit()

 */
