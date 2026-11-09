/// Insert a new anidb entry into the database; returns the new guid.
///
/// NOTE: the schema dump declares `mm_metadata_anime_media_id` as integer, but
/// the Python original treats it as jsonb (see the `->'anidb'` lookups below).
/// Ported as jsonb to match the queries; confirm against the live database.
#[allow(clippy::too_many_arguments)]
pub async fn mk_lib_database_metadata_anime_insert(
    sqlx_pool: &sqlx::PgPool,
    ani_media_id_json: &serde_json::Value,
    ani_name: &str,
    ani_json: &serde_json::Value,
    ani_image_local: &serde_json::Value,
    ani_user_json: &serde_json::Value,
    mapping_data: &serde_json::Value,
    before_data: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_anime (mm_metadata_anime_guid,
        mm_metadata_anime_media_id, mm_metadata_anime_name, mm_metadata_anime_json,
        mm_metadata_anime_localimage, mm_metadata_anime_user_json,
        mm_metadata_anime_mapping, mm_metadata_anime_mapping_before)
        values ($1,$2,$3,$4,$5,$6,$7,$8)"#,
    )
    .bind(new_guid)
    .bind(ani_media_id_json)
    .bind(ani_name)
    .bind(ani_json)
    .bind(ani_image_local)
    .bind(ani_user_json)
    .bind(mapping_data)
    .bind(before_data)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Search for an anime title; returns the guid of the first match.
pub async fn mk_lib_database_metadata_anime_guid_by_name(
    sqlx_pool: &sqlx::PgPool,
    title_to_search: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    // TODO hit movie and tv db's as well?
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"select mm_metadata_anime_guid from mm_metadata_anime
        where mm_metadata_anime_name = $1"#,
    )
    .bind(title_to_search)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.map(|row| row.0))
}

/// Update the media id json from scudlee data (matched by anidb id).
pub async fn mk_lib_database_metadata_anime_update_media_id(
    sqlx_pool: &sqlx::PgPool,
    media_id_json: &serde_json::Value,
    mapping_json: &serde_json::Value,
    mapping_before: &str,
) -> Result<(), sqlx::Error> {
    // The Python original also logged media_id_json to Elasticsearch; this crate
    // has no ES client.
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_anime
        set mm_metadata_anime_media_id = $1,
            mm_metadata_anime_mapping = $2,
            mm_metadata_anime_mapping_before = $3
        where mm_metadata_anime_media_id->'anidb' ? $4"#,
    )
    .bind(media_id_json)
    .bind(mapping_json)
    .bind(mapping_before)
    .bind(&media_id_json["anidb"])
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Anime guid by anidb id.
pub async fn mk_lib_database_metadata_anime_guid_by_anidb_id(
    sqlx_pool: &sqlx::PgPool,
    anidb_id: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"select mm_metadata_anime_guid from mm_metadata_anime
        where mm_metadata_anime_media_id->'anidb' ? $1"#,
    )
    .bind(anidb_id)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.map(|row| row.0))
}
