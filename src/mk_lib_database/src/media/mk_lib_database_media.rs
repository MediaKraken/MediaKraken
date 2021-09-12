use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_media_insert(pool: &sqlx::PgPool,
                                          mm_media_guid: Uuid,
                                          mm_media_class_enum: i16,
                                          mm_media_path: String,
                                          mm_media_metadata_guid: Uuid,
                                          mm_media_ffprobe_json: serde_json::Value,
                                          mm_media_json: serde_json::Value)
                                          -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_media (mm_media_guid, mm_media_class_enum, \
         mm_media_path, mm_media_metadata_guid, mm_media_ffprobe_json, mm_media_json) \
         values ($1, $2, $3, $4, $5, $6)")
        .bind(mm_media_guid)
        .bind(mm_media_class_enum)
        .bind(mm_media_path)
        .bind(mm_media_metadata_guid)
        .bind(mm_media_ffprobe_json)
        .bind(mm_media_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
