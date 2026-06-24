use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStatusUpdatePayload {
    pub guid: uuid::Uuid,
    pub favorite: bool,
    pub watched: bool,
    pub good: bool,
    pub bad: bool,
    pub trash: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_media_status_update_payload_serialization() {
        let payload = MediaStatusUpdatePayload {
            guid: Uuid::nil(),
            favorite: true,
            watched: false,
            good: true,
            bad: false,
            trash: false,
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"guid\""));
        assert!(json.contains("\"favorite\":true"));
        assert!(json.contains("\"watched\":false"));
    }

    #[test]
    fn test_media_status_update_payload_deserialization() {
        let json = r#"{"guid":"00000000-0000-0000-0000-000000000000","favorite":true,"watched":false,"good":true,"bad":false,"trash":false}"#;
        let payload: MediaStatusUpdatePayload = serde_json::from_str(json).unwrap();
        assert!(payload.favorite);
        assert!(!payload.watched);
        assert!(payload.good);
    }

    #[test]
    fn test_media_status_update_payload_all_false() {
        let payload = MediaStatusUpdatePayload {
            guid: Uuid::new_v4(),
            favorite: false,
            watched: false,
            good: false,
            bad: false,
            trash: false,
        };
        assert!(!payload.favorite);
        assert!(!payload.watched);
        assert!(!payload.good);
        assert!(!payload.bad);
        assert!(!payload.trash);
    }

    #[test]
    fn test_media_status_update_payload_all_true() {
        let payload = MediaStatusUpdatePayload {
            guid: Uuid::new_v4(),
            favorite: true,
            watched: true,
            good: true,
            bad: true,
            trash: true,
        };
        assert!(payload.favorite);
        assert!(payload.watched);
        assert!(payload.good);
        assert!(payload.bad);
        assert!(payload.trash);
    }

    #[test]
    fn test_media_status_update_payload_clone() {
        let payload = MediaStatusUpdatePayload {
            guid: Uuid::new_v4(),
            favorite: true,
            watched: false,
            good: false,
            bad: false,
            trash: false,
        };
        let cloned = payload.clone();
        assert_eq!(payload.guid, cloned.guid);
        assert_eq!(payload.favorite, cloned.favorite);
    }
}

pub async fn mk_lib_database_open_pool(
    pool_connections: u32,
    connection_timeout: u64,
) -> Result<(sqlx::PgPool, sqlx::PgPool), Box<dyn std::error::Error + Send + Sync>> {
    let db_user = env::var("POSTGRES_USER").map_err(|e| format!("POSTGRES_USER not set: {e}"))?;
    let db_pass = env::var("POSTGRES_PASSWORD").map_err(|e| format!("POSTGRES_PASSWORD not set: {e}"))?;
    let create_pool = |host: &str| {
        let connection_string = format!(
            "postgresql://{}:{}@{}:5432/mkdatabase?sslmode=prefer",
            db_user,
            encode(&db_pass),
            host
        );
        PgPoolOptions::new()
            .max_connections(pool_connections)
            .acquire_timeout(Duration::from_secs(connection_timeout))
            .max_lifetime(Duration::from_secs(1800))
            .connect_lazy(&connection_string)
    };
    let sqlx_pool_rw = create_pool("pgcluster-with-metrics-pgbouncer-rw.cnpg-system")
        .map_err(|e| format!("failed to create rw pool: {e}"))?;
    let sqlx_pool_ro = create_pool("pgcluster-with-metrics-pgbouncer-ro.cnpg-system")
        .map_err(|e| format!("failed to create ro pool: {e}"))?;
    Ok((sqlx_pool_rw, sqlx_pool_ro))
}
