use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicList {
    pub mm_metadata_album_id: i32,
    pub mm_metadata_album_name: String,
    pub mm_metadata_album_artist: String,
}

pub async fn mk_lib_database_metadata_music_album_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != String::new() {
        let row: (i64,) =
            sqlx::query_as(r#"select count(*) from musicbrainz.release where name % $1"#)
                .bind(search_value)
                .fetch_one(sqlx_pool)
                .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from musicbrainz.release"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_music_album_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicList>, sqlx::Error> {
    // TODO, only grab the poster locale from json
    // TODO order by release year
    if search_value != String::new() {
        sqlx::query_as(
            r#"select release.id as mm_metadata_album_id, release.name as mm_metadata_album_name, artist.name as mm_metadata_album_artist from musicbrainz.release, musicbrainz.artist where artist.id = artist_credit and release.name % $1 order by LOWER(release.name), LOWER(artist.name) offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as(
            r#"select release.id as mm_metadata_album_id, release.name as mm_metadata_album_name, artist.name as mm_metadata_album_artist from musicbrainz.release, musicbrainz.artist where artist.id = artist_credit order by LOWER(release.name), LOWER(artist.name) offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}
