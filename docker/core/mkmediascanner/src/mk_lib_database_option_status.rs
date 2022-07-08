#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_option_read(
    pool: &sqlx::PgPool,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as("select mm_options_json from mm_options_and_status")
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_status_read(
    pool: &sqlx::PgPool,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as("select mm_status_json from mm_options_and_status")
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_option_status_read(
    pool: &sqlx::PgPool,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query(
        "select mm_options_json, mm_status_json \
        from mm_options_and_status",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn mk_lib_database_option_update(
    pool: &sqlx::PgPool,
    option_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_options_and_status set mm_options_json = $1")
        .bind(option_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_option_status_update(
    pool: &sqlx::PgPool,
    option_json: serde_json::Value,
    status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_options_and_status set mm_options_json = $1, mm_status_json = $2")
        .bind(option_json)
        .bind(status_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_status_update_scan(
    pool: &sqlx::PgPool,
    status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_options_and_status set mm_status_json = $1")
        .bind(status_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

/*
// TODO port query
def db_opt_status_update_scan_rec(self, dir_path, scan_status, scan_percent):
    """
    Update scan data
    """
    self.db_cursor.execute('select mm_status_json'
                           ' from mm_options_and_status')
    # will always have the one record
    status_json = self.db_cursor.fetchone()['mm_status_json']
    status_json.update(
        {'Scan': {dir_path: {'Status': scan_status, 'Pct': scan_percent}}})

    # how about have the status on the lib record itself
    # then in own thread....no, read to update....just update
    # so faster
    #    json_data = self.db_cursor.fetchone()[0]
    #    json_data.update({'UserStats':{user_id:{'Watched':status_bool}}})
    #    json_data = json.dumps(json_data)

    # no need for where clause as it's only the one record
    self.db_cursor.execute('update mm_options_and_status'
                           ' set mm_status_json = $1',
                           (json.dumps(status_json),))
    # 'update objects set mm_options_and_status=jsonb_set(mm_options_and_status,
    # '{name}', '"Mary"', true)'
    self.db_commit()

 */
