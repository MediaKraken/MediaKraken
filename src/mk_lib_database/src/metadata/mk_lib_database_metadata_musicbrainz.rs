//! MusicBrainz (fullexport) lookups backing the mkwebaxum metadata music
//! pages (bands -> albums -> tracks).
//!
//! The imported dump lives in the `musicbrainz` schema (populated by the
//! `mkmusicbrainz` service). The MediaKraken-specific objects the pages
//! rely on are created by that service's `create_mkstack_objects` stage:
//!   - the `mkstack_bands` materialized view (artist credits that have at
//!     least one release, with their release count) plus its indexes
//!   - `mkstack_medium_release_idx` / `mkstack_track_medium_idx` (FK
//!     indexes the upstream dump schema does not create)
//!
//! The dump schema is a reduced "replicate" schema: releases carry no date
//! columns and there is no `tracklist` table (track order is
//! `medium.position`, then `track.position`).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// One band list row. A band is an `artist_credit` that has at least one
/// release; `mb_band_release_count` is how many releases are credited to it.
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicBrainzBand {
    pub mb_band_id: i32,
    pub mb_band_name: String,
    pub mb_band_release_count: i64,
}

/// Count bands matching the list filter (`starts_with` is "", "#", or a
/// single letter).
pub async fn mk_lib_database_metadata_musicbrainz_band_count(
    sqlx_pool: &sqlx::PgPool,
    starts_with: &str,
) -> Result<i64, sqlx::Error> {
    if starts_with.is_empty() {
        let row: (i64,) = sqlx::query_as("select count(*) from musicbrainz.mkstack_bands")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else if starts_with == "#" {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*)
               from musicbrainz.mkstack_bands
               where left(lower(name), 1) !~ '^[a-z0-9]'"#,
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*)
               from musicbrainz.mkstack_bands
               where lower(name) like lower($1) || '%'"#,
        )
        .bind(starts_with)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

/// Read bands matching the list filter, paginated. Without a filter the
/// most prolific bands come first; with a filter they are name ordered.
pub async fn mk_lib_database_metadata_musicbrainz_band_read(
    sqlx_pool: &sqlx::PgPool,
    starts_with: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicBrainzBand>, sqlx::Error> {
    if starts_with.is_empty() {
        sqlx::query_as(
            r#"select id as mb_band_id, name as mb_band_name, release_count as mb_band_release_count
               from musicbrainz.mkstack_bands
               order by release_count desc, name
               offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else if starts_with == "#" {
        sqlx::query_as(
            r#"select id as mb_band_id, name as mb_band_name, release_count as mb_band_release_count
               from musicbrainz.mkstack_bands
               where left(lower(name), 1) !~ '^[a-z0-9]$'
               order by lower(name), id
               offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select id as mb_band_id, name as mb_band_name, release_count as mb_band_release_count
               from musicbrainz.mkstack_bands
               where lower(name) like lower($1) || '%'
               order by lower(name), id
               offset $2 limit $3"#,
        )
        .bind(starts_with)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

/// Fetch one band (artist credit) by id, or None when it has no releases
/// (or does not exist at all).
pub async fn mk_lib_database_metadata_musicbrainz_band(
    sqlx_pool: &sqlx::PgPool,
    band_id: i32,
) -> Result<Option<DBMetaMusicBrainzBand>, sqlx::Error> {
    sqlx::query_as(
        r#"select id as mb_band_id, name as mb_band_name, release_count as mb_band_release_count
           from musicbrainz.mkstack_bands
           where id = $1"#,
    )
    .bind(band_id)
    .fetch_optional(sqlx_pool)
    .await
}

/// One album (release) row on a band's album list.
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicBrainzAlbum {
    pub mb_album_id: i32,
    pub mb_album_name: String,
    pub mb_album_release_group_name: String,
    pub mb_album_release_group_type: String,
    pub mb_album_release_status: String,
    pub mb_album_language: String,
    pub mb_album_comment: String,
}

/// Count a band's releases (its "albums").
pub async fn mk_lib_database_metadata_musicbrainz_album_count(
    sqlx_pool: &sqlx::PgPool,
    band_id: i32,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) =
        sqlx::query_as("select count(*) from musicbrainz.release where artist_credit = $1")
            .bind(band_id)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}

/// Read one band's releases, ordered by release group name then edition
/// name, paginated.
pub async fn mk_lib_database_metadata_musicbrainz_album_read(
    sqlx_pool: &sqlx::PgPool,
    band_id: i32,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicBrainzAlbum>, sqlx::Error> {
    sqlx::query_as(
        r#"select r.id as mb_album_id,
                  r.name as mb_album_name,
                  coalesce(rg.name, '') as mb_album_release_group_name,
                  coalesce(rgt.name, '') as mb_album_release_group_type,
                  coalesce(rs.name, '') as mb_album_release_status,
                  coalesce(lang.name, '') as mb_album_language,
                  r.comment as mb_album_comment
           from musicbrainz.release r
           left join musicbrainz.release_group rg on rg.id = r.release_group
           left join musicbrainz.release_group_primary_type rgt on rgt.id = rg.type
           left join musicbrainz.release_status rs on rs.id = r.status
           left join musicbrainz.language lang on lang.id = r.language
           where r.artist_credit = $1
           order by lower(coalesce(rg.name, r.name)), lower(r.name), r.id
           offset $2 limit $3"#,
    )
    .bind(band_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await
}

/// Header data for one album (release) detail page.
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicBrainzAlbumDetail {
    pub mb_album_id: i32,
    pub mb_album_name: String,
    pub mb_album_band_id: i32,
    pub mb_album_band_name: String,
    pub mb_album_release_group_name: String,
    pub mb_album_release_group_type: String,
    pub mb_album_release_status: String,
    pub mb_album_language: String,
    pub mb_album_comment: String,
    pub mb_album_barcode: String,
}

/// Fetch one album (release) with its artist credit, or None when the id
/// does not exist.
pub async fn mk_lib_database_metadata_musicbrainz_album(
    sqlx_pool: &sqlx::PgPool,
    album_id: i32,
) -> Result<Option<DBMetaMusicBrainzAlbumDetail>, sqlx::Error> {
    sqlx::query_as(
        r#"select r.id as mb_album_id,
                  r.name as mb_album_name,
                  ac.id as mb_album_band_id,
                  ac.name as mb_album_band_name,
                  coalesce(rg.name, '') as mb_album_release_group_name,
                  coalesce(rgt.name, '') as mb_album_release_group_type,
                  coalesce(rs.name, '') as mb_album_release_status,
                  coalesce(lang.name, '') as mb_album_language,
                  r.comment as mb_album_comment,
                  coalesce(r.barcode, '') as mb_album_barcode
           from musicbrainz.release r
           join musicbrainz.artist_credit ac on ac.id = r.artist_credit
           left join musicbrainz.release_group rg on rg.id = r.release_group
           left join musicbrainz.release_group_primary_type rgt on rgt.id = rg.type
           left join musicbrainz.release_status rs on rs.id = r.status
           left join musicbrainz.language lang on lang.id = r.language
           where r.id = $1"#,
    )
    .bind(album_id)
    .fetch_optional(sqlx_pool)
    .await
}

/// One track row of an album's tracklist.
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicBrainzTrack {
    pub mb_medium_position: i32,
    pub mb_medium_name: String,
    pub mb_medium_format: String,
    pub mb_track_number: String,
    pub mb_track_name: String,
    pub mb_track_length_ms: Option<i32>,
    pub mb_track_artist: String,
}

/// Read a release's full tracklist (data tracks excluded), in medium
/// order.
pub async fn mk_lib_database_metadata_musicbrainz_album_tracks(
    sqlx_pool: &sqlx::PgPool,
    album_id: i32,
) -> Result<Vec<DBMetaMusicBrainzTrack>, sqlx::Error> {
    sqlx::query_as(
        r#"select m.position as mb_medium_position,
                  m.name as mb_medium_name,
                  coalesce(mf.name, '') as mb_medium_format,
                  t.number as mb_track_number,
                  t.name as mb_track_name,
                  t.length as mb_track_length_ms,
                  coalesce(tac.name, '') as mb_track_artist
           from musicbrainz.medium m
           join musicbrainz.track t on t.medium = m.id
           left join musicbrainz.medium_format mf on mf.id = m.format
           left join musicbrainz.artist_credit tac on tac.id = t.artist_credit
           where m.release = $1
             and not t.is_data_track
           order by m.position, t.position"#,
    )
    .bind(album_id)
    .fetch_all(sqlx_pool)
    .await
}
