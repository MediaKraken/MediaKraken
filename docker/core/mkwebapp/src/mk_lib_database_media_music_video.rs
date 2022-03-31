use sqlx::postgres::PgRow;
use uuid::Uuid;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_media_music_video_count(pool: &sqlx::PgPool,
                                                     search_value: String)
                                                     -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_music_video, mm_media \
            where mm_media_metadata_guid = mm_metadata_music_video_guid group \
            and mm_media_music_video_song % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_music_video, mm_media \
            where mm_media_metadata_guid = mm_metadata_music_video_guid")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}
