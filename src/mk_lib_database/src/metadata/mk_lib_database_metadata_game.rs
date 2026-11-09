use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_game_detail(
    sqlx_pool: &sqlx::PgPool,
    game_uuid: String,
) -> Result<(uuid::Uuid, serde_json::Value), sqlx::Error> {
    let row: (uuid::Uuid, serde_json::Value) = sqlx::query_as(
        r#"select gi_game_info_system_id, gi_game_info_json from mm_metadata_game_software_info where gi_game_info_id = $1"#,
    )
    .bind(game_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_game_by_sha1(
    sqlx_pool: &sqlx::PgPool,
    sha1_hash: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select gi_game_info_id from mm_metadata_game_software_info where gi_game_info_sha1 = $1"#,
    )
    .bind(sha1_hash)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_by_blake3(
    sqlx_pool: &sqlx::PgPool,
    blake3_hash: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select gi_game_info_id from mm_metadata_game_software_info where gi_game_info_blake3 = $1"#,
    )
    .bind(blake3_hash)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    starts_with: String,
    genre: String,
    status_filter: String,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*)
           from mm_metadata_game_software_info
           where
             ($1 = '' or gi_game_info_name &@ $1)
             and (
               $2 = ''
               or ($2 = '#' and left(lower(gi_game_info_name), 1) !~ '^[a-z0-9]$')
               or ($2 <> '#' and lower(gi_game_info_name) like lower($2) || '%')
             )
             and ($3 = '' or lower(coalesce(gi_gc_category, '')) = lower($3))
             and (
               $4 = ''
               or ($4 = 'favorite' and coalesce((gi_game_info_json->'user_status'->>'favorite')::boolean, false))
               or ($4 = 'watched' and coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'unwatched' and not coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'good' and coalesce((gi_game_info_json->'user_status'->>'good')::boolean, false))
               or ($4 = 'bad' and coalesce((gi_game_info_json->'user_status'->>'bad')::boolean, false))
               or ($4 = 'trash' and coalesce((gi_game_info_json->'user_status'->>'trash')::boolean, false))
             )"#,
    )
    .bind(search_value)
    .bind(starts_with)
    .bind(genre)
    .bind(status_filter)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameList {
    pub gi_game_info_id: uuid::Uuid,
    pub gi_game_info_short_name: String,
    pub gi_game_info_name: String,
    pub gi_year: Option<String>,
    pub gi_game_info_localimage: Option<serde_json::Value>,
    pub gs_game_system_name: String,
    pub gi_gc_category: Option<String>,
}

pub async fn mk_lib_database_metadata_game_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    starts_with: String,
    genre: String,
    status_filter: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaGameList>, sqlx::Error> {
    sqlx::query_as(
        r#"select gi_game_info_id,
           gi_game_info_short_name,
           gi_game_info_name,
           gi_game_info_json->'machine'->>'year' as gi_year,
           gi_game_info_localimage,
           gs_game_system_name,
           gi_gc_category
           from mm_metadata_game_software_info, mm_metadata_game_systems_info
           where gi_game_info_system_id = gs_game_system_id
           and ($1 = '' or gi_game_info_name &@ $1)
           and (
               $2 = ''
               or ($2 = '#' and left(lower(gi_game_info_name), 1) !~ '^[a-z0-9]$')
               or ($2 <> '#' and lower(gi_game_info_name) like lower($2) || '%')
           )
           and ($3 = '' or lower(coalesce(gi_gc_category, '')) = lower($3))
           and (
               $4 = ''
               or ($4 = 'favorite' and coalesce((gi_game_info_json->'user_status'->>'favorite')::boolean, false))
               or ($4 = 'watched' and coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'unwatched' and not coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'good' and coalesce((gi_game_info_json->'user_status'->>'good')::boolean, false))
               or ($4 = 'bad' and coalesce((gi_game_info_json->'user_status'->>'bad')::boolean, false))
               or ($4 = 'trash' and coalesce((gi_game_info_json->'user_status'->>'trash')::boolean, false))
           )
           order by gi_game_info_name, gi_year
           offset $5
           limit $6"#,
    )
    .bind(search_value)
    .bind(starts_with)
    .bind(genre)
    .bind(status_filter)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await
}

pub async fn mk_lib_database_metadata_game_genres(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"select distinct gi_gc_category
           from mm_metadata_game_software_info
           where coalesce(gi_gc_category, '') <> ''
           order by gi_gc_category"#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.0).collect())
}

pub async fn mk_lib_database_metadata_game_uuid_by_name_and_system(
    sqlx_pool: &sqlx::PgPool,
    game_name: String,
    game_system_short_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    if !game_system_short_name.is_empty() {
        let row: (uuid::Uuid,) = sqlx::query_as(
            r#"select gi_id from mm_metadata_game_software_info where gi_game_info_name = $1 and game_system_short_name = $2 limit 1"#,
        )
        .bind(game_name)
        .bind(game_system_short_name)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (uuid::Uuid,) = sqlx::query_as(
            r#"select gi_id from mm_metadata_game_software_info where gi_game_info_name = $1 and gi_game_info_system_id IS NULL limit 1"#,
        )
        .bind(game_name)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameNameMatchList {
    gi_id: uuid::Uuid,
    gi_game_info_json: String,
}

pub async fn mk_lib_database_metadata_game_by_name_and_system(
    sqlx_pool: &sqlx::PgPool,
    game_name: String,
    game_system_short_name: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaGameNameMatchList>, sqlx::Error> {
    if !game_system_short_name.is_empty() {
        sqlx::query_as(
            r#"select gi_id, gi_game_info_json from mm_metadata_game_software_info, mm_metadata_game_systems_info where gi_game_info_name = $1 and gs_game_system_name = $2 and gi_game_info_system_id = gs_game_system_id offset $3 limit $4"#,
        )
        .bind(game_name)
        .bind(game_system_short_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select gi_id, gi_game_info_json from mm_metadata_game_software_info where gi_game_info_name = $1 and gi_game_info_system_id IS NULL offset $2 limit $3"#,
        )
        .bind(game_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_game_insert(
    sqlx_pool: &sqlx::PgPool,
    game_system_id: Uuid,
    game_short_name: String,
    game_name: String,
    game_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_game_software_info(gi_game_info_id, gi_game_info_system_id, gi_game_info_short_name, gi_game_info_name, gi_game_info_json) values ($1, $2, $3, $4, $5)"#,
    )
    .bind(new_guid)
    .bind(game_system_id)
    .bind(game_short_name)
    .bind(game_name)
    .bind(&game_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Update a game's json found by system guid, short name and name.
pub async fn mk_lib_database_metadata_game_update(
    sqlx_pool: &sqlx::PgPool,
    game_json: &serde_json::Value,
    game_system_id: uuid::Uuid,
    game_short_name: &str,
    game_name: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_game_software_info
        set gi_game_info_json = $1
        where gi_game_info_system_id = $2
        and gi_game_info_short_name = $3
        and gi_game_info_name = $4"#,
    )
    .bind(game_json)
    .bind(game_system_id)
    .bind(game_short_name)
    .bind(game_name)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Game info (id, system id, json) by name or short name.
pub async fn mk_lib_database_metadata_game_by_name(
    sqlx_pool: &sqlx::PgPool,
    game_name: &str,
    game_short_name: &str,
) -> Result<Vec<(uuid::Uuid, uuid::Uuid, serde_json::Value)>, sqlx::Error> {
    let rows: Vec<(uuid::Uuid, uuid::Uuid, serde_json::Value)> = sqlx::query_as(
        r#"select gi_game_info_id, gi_game_info_system_id, gi_game_info_json
        from mm_metadata_game_software_info
        where gi_game_info_name = $1
        or gi_game_info_short_name = $2"#,
    )
    .bind(game_name)
    .bind(game_short_name)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

/// Update a game's json by its guid.
pub async fn mk_lib_database_metadata_game_update_by_guid(
    sqlx_pool: &sqlx::PgPool,
    game_json: &serde_json::Value,
    game_guid: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_game_software_info
        set gi_game_info_json = $1
        where gi_game_info_id = $2"#,
    )
    .bind(game_json)
    .bind(game_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Count of games for a game system guid.
pub async fn mk_lib_database_metadata_game_by_system_count(
    sqlx_pool: &sqlx::PgPool,
    system_guid: uuid::Uuid,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_metadata_game_software_info, mm_metadata_game_systems_info
        where gi_game_info_system_id = gs_game_system_id
        and gs_game_system_id = $1"#,
    )
    .bind(system_guid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Games for a game system guid, all columns, with pagination.
///
/// NOTE: the Python original had `offset $2, limit $3` (comma typo); ported
/// as `offset $2 limit $3`, and used fetchone-with-try/except (returns at most
/// one row); ported as fetch_all.
pub async fn mk_lib_database_metadata_game_by_system(
    sqlx_pool: &sqlx::PgPool,
    system_guid: uuid::Uuid,
    offset: i64,
    records: Option<i64>,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = match records {
        Some(records) => {
            sqlx::query(
                r#"select * from mm_metadata_game_software_info, mm_metadata_game_systems_info
                where gi_game_info_system_id = gs_game_system_id
                and gs_game_system_id = $1
                offset $2 limit $3"#,
            )
            .bind(system_guid)
            .bind(offset)
            .bind(records)
            .fetch_all(sqlx_pool)
            .await?
        }
        None => {
            sqlx::query(
                r#"select * from mm_metadata_game_software_info, mm_metadata_game_systems_info
                where gi_game_info_system_id = gs_game_system_id
                and gs_game_system_id = $1
                offset $2"#,
            )
            .bind(system_guid)
            .bind(offset)
            .fetch_all(sqlx_pool)
            .await?
        }
    };
    Ok(rows)
}

/// Find a random game image of the requested type (e.g. "Poster").
///
/// NOTE: the Python original used double-quoted jsonb keys
/// (`gi_game_info_json->"Images"->"thegamesdb"->>"X"`), which is not valid
/// Postgres jsonb syntax; ported as single-quoted keys.
/// The image type is embedded in a JSON path in the SQL, so it is restricted
/// to `[A-Za-z0-9_-]`; the Python original was injectable ("little bobby
/// tables").
pub async fn mk_lib_database_metadata_game_image_random(
    sqlx_pool: &sqlx::PgPool,
    return_image_type: &str,
) -> Result<Option<(serde_json::Value, uuid::Uuid)>, sqlx::Error> {
    if !is_valid_image_type(return_image_type) {
        return Err(sqlx::Error::Configuration(
            format!("invalid image type: {return_image_type}").into(),
        ));
    }
    let sql = format!(
        r#"select gi_game_info_json->'Images'->'thegamesdb'->{return_image_type} as image_json,
        gi_game_info_id
        from mm_media, mm_metadata_game_software_info
        where mm_media_metadata_guid = gi_game_info_id
        and (gi_game_info_json->'Images'->'thegamesdb'->>'{return_image_type}')::text != 'null'
        order by random() limit 1"#,
    );
    let row: Option<(serde_json::Value, uuid::Uuid)> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .fetch_optional(sqlx_pool)
        .await?;
    Ok(row)
}

/// Game category guid by category name.
///
/// NOTE: `mm_game_category` is not in the schema dump but is used by
/// `mk_lib_database_metadata_game_category_insert`; confirm against the
/// live database.
pub async fn mk_lib_database_metadata_game_category_by_name(
    sqlx_pool: &sqlx::PgPool,
    category_name: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row: Option<(uuid::Uuid,)> =
        sqlx::query_as(r#"select gc_id from mm_game_category where gc_category = $1"#)
            .bind(category_name)
            .fetch_optional(sqlx_pool)
            .await?;
    Ok(row.map(|row| row.0))
}

/// True when the image type is safe to embed in a SQL JSON path.
fn is_valid_image_type(image_type: &str) -> bool {
    !image_type.is_empty()
        && image_type
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
