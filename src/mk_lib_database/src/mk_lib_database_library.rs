use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryList {
	mm_media_dir_guid: uuid::Uuid,
	mm_media_dir_path: String,
}

pub async fn mk_lib_database_library_read(pool: &sqlx::PgPool)
                                          -> Result<Vec<DBLibraryList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_media_dir_guid, mm_media_dir_path \
        from mm_library_dir");
    let table_rows: Vec<DBLibraryList> = select_query
		.map(|row: PgRow| DBLibraryList {
			mm_media_dir_guid: row.get("mm_media_dir_guid"),
			mm_media_dir_path: row.get("mm_media_dir_path"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_library_path_audit(pool: &sqlx::PgPool)
                                                -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_media_dir_guid, mm_media_dir_path, \
        mm_media_dir_class_enum, \
        mm_media_dir_last_scanned from mm_library_dir")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryPathStatus {
	mm_media_dir_path: String,
	mm_media_dir_status: String,
}

pub async fn mk_lib_database_library_path_status(pool: &sqlx::PgPool)
                                                 -> Result<Vec<DBLibraryPathStatus>, sqlx::Error> {
    let select_query = sqlx::query("select mm_media_dir_path, mm_media_dir_status \
        from mm_library_dir where mm_media_dir_status IS NOT NULL \
        order by mm_media_dir_path");
    let table_rows: Vec<DBLibraryPathStatus> = select_query
		.map(|row: PgRow| DBLibraryPathStatus {
			mm_media_dir_path: row.get("mm_media_dir_path"),
			mm_media_dir_status: row.get("mm_media_dir_status"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_library_path_status_update(pool: &sqlx::PgPool,
                                                        library_uuid: uuid::Uuid,
                                                        library_status_json: serde_json::Value)
                                                        -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_library_dir set mm_media_dir_status = $1 \
        where mm_media_dir_guid = $2")
        .bind(library_status_json)
        .bind(library_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_library_path_timestamp_update(pool: &sqlx::PgPool,
                                                           library_uuid: uuid::Uuid)
                                                           -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_library_dir set mm_media_dir_last_scanned = NOW() \
        where mm_media_dir_guid = $1")
        .bind(library_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_library_file_exists(pool: &sqlx::PgPool,
                                                 file_name: String)
                                                 -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_media \
        where mm_media_path = $1 limit 1) \
        as found_record limit 1")
        .bind(file_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
