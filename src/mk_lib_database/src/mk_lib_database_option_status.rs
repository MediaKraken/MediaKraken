use serde::Deserialize;
use sqlx::postgres::PgRow;

#[derive(Deserialize, Debug)]
pub struct APIJson {
    pub themoviedb: String,
    pub musicbrainz: Option<String>,
    pub thesportsdb: String,
    pub upcitemdb: Option<String>,
    pub barcodespider: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_json_deserialization_with_all_fields() {
        let json = r#"{"themoviedb":"key1","musicbrainz":"key2","thesportsdb":"key3","upcitemdb":"key4","barcodespider":"key5"}"#;
        let api: APIJson = serde_json::from_str(json).unwrap();
        assert_eq!(api.themoviedb, "key1");
        assert_eq!(api.musicbrainz, Some("key2".to_string()));
        assert_eq!(api.thesportsdb, "key3");
        assert_eq!(api.upcitemdb, Some("key4".to_string()));
        assert_eq!(api.barcodespider, Some("key5".to_string()));
    }

    #[test]
    fn test_api_json_deserialization_with_optional_nils() {
        let json = r#"{"themoviedb":"key1","musicbrainz":null,"thesportsdb":"key3","upcitemdb":null,"barcodespider":null}"#;
        let api: APIJson = serde_json::from_str(json).unwrap();
        assert_eq!(api.themoviedb, "key1");
        assert_eq!(api.musicbrainz, None);
        assert_eq!(api.thesportsdb, "key3");
        assert_eq!(api.upcitemdb, None);
        assert_eq!(api.barcodespider, None);
    }

    #[test]
    fn test_api_json_deserialization_missing_optional_fields() {
        let json = r#"{"themoviedb":"key1","thesportsdb":"key3"}"#;
        let api: APIJson = serde_json::from_str(json).unwrap();
        assert_eq!(api.themoviedb, "key1");
        assert_eq!(api.musicbrainz, None);
        assert_eq!(api.thesportsdb, "key3");
        assert_eq!(api.upcitemdb, None);
        assert_eq!(api.barcodespider, None);
    }

    #[test]
    fn test_api_json_debug() {
        let json = r#"{"themoviedb":"key1","thesportsdb":"key3"}"#;
        let api: APIJson = serde_json::from_str(json).unwrap();
        let debug_str = format!("{:?}", api);
        assert!(debug_str.contains("APIJson"));
    }
}

pub async fn mk_lib_database_option_api_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"select mm_options_json->'API' from mm_options_and_status"#)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_option_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"select mm_options_json from mm_options_and_status"#)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_status_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"select mm_status_json from mm_options_and_status"#)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_option_status_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<PgRow, sqlx::Error> {
    let rows = sqlx::query(r#"select mm_options_json, mm_status_json from mm_options_and_status"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_option_update(
    sqlx_pool: &sqlx::PgPool,
    option_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"update mm_options_and_status set mm_options_json = $1"#)
        .bind(option_json)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_option_status_update(
    sqlx_pool: &sqlx::PgPool,
    option_json: serde_json::Value,
    status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"update mm_options_and_status set mm_options_json = $1, mm_status_json = $2"#)
        .bind(option_json)
        .bind(status_json)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_status_update_scan(
    sqlx_pool: &sqlx::PgPool,
    status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    // no need for where clause as it's only the one record
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"update mm_options_and_status set mm_status_json = $1"#)
        .bind(status_json)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
