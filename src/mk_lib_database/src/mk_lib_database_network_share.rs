use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

/// Returns the symmetric key pgcrypto should use for share-auth passwords.
///
/// Reads `MK_SHARE_AUTH_KEY` from the environment; falls back to the legacy
/// key for backward compatibility with databases migrated before this was
/// configurable. Set the env var in production and rotate existing rows.
fn share_auth_key() -> String {
    std::env::var("MK_SHARE_AUTH_KEY").unwrap_or_else(|_| "fake-strong-key".to_string())
}

/// Extract the share name from a network share path.
///
/// Accepts either backslash UNC (`\\host\share\...`, the format nmap's
/// `smb-enum-shares` emits) or forward-slash UNC (`//host/share/...`). The
/// host segment is stripped — it is stored separately in
/// `mm_network_share_ip` — and any deeper subpath is dropped, so only the
/// share name itself is returned. A bare `share` with no UNC prefix is also
/// accepted and returned unchanged.
///
/// IP-address-shaped values are rejected: when smb-enum-shares cannot
/// enumerate anonymously it can surface a host-keyed entry, which would
/// otherwise produce a `//host/host` URI when browsing.
pub fn parse_share_name(network_share_path: &str) -> Option<String> {
    let normalized = network_share_path.replace('\\', "/");
    let had_unc_prefix = normalized.starts_with("//");
    let mut segments = normalized.split('/').filter(|s| !s.is_empty());
    if had_unc_prefix {
        segments.next()?;
    }
    let candidate = segments.next()?.to_owned();
    if candidate.parse::<std::net::IpAddr>().is_ok() {
        return None;
    }
    Some(candidate)
}

pub async fn mk_lib_database_network_share_exists(
    sqlx_pool: &sqlx::PgPool,
    network_share_ip: std::net::IpAddr,
    network_share_path: &str,
) -> Result<bool, sqlx::Error> {
    let share_path = parse_share_name(network_share_path).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "invalid share path (expected '//host/share' or '\\\\host\\share', got {network_share_path:?})"
        ))
    })?;
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_network_shares where mm_network_share_ip = $1
        and mm_network_share_path = $2 limit 1) as found_record limit 1"#,
    )
    .bind(network_share_ip)
    .bind(share_path)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_network_share_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_network_shares"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBShareAuthUserList {
    pub mm_share_auth_guid: uuid::Uuid,
    pub mm_share_auth_user: String,
}

pub async fn mk_lib_database_network_share_user_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBShareAuthUserList>, sqlx::Error> {
    let table_rows: Vec<DBShareAuthUserList> = sqlx::query_as(
        r#"select mm_share_auth_guid, mm_share_auth_user 
        from mm_share_auth order by mm_share_auth_user"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_network_share_detail(
    sqlx_pool: &sqlx::PgPool,
    share_guid: uuid::Uuid,
) -> Result<DBShareList, sqlx::Error> {
    let table_row: DBShareList = sqlx::query_as(
        r#"select mm_network_share_guid, mm_network_share_ip, mm_network_share_path,
        mm_network_share_comment, mm_share_auth_user,
        pgp_sym_decrypt(mm_share_auth_password::bytea, $2) AS mm_share_auth_password,
        mm_network_share_version, mm_network_share_workgroup
        from mm_network_shares
        LEFT JOIN mm_share_auth ON mm_network_shares.mm_network_share_user_guid = mm_share_auth.mm_share_auth_guid
        where mm_network_share_guid = $1"#,
    )
    .bind(share_guid)
    .bind(share_auth_key())
    .fetch_one(sqlx_pool)
    .await?;
    Ok(table_row)
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct DBShareList {
    pub mm_network_share_guid: uuid::Uuid,
    pub mm_network_share_ip: std::net::IpAddr,
    pub mm_network_share_path: String,
    pub mm_network_share_comment: String,
    pub mm_network_share_version: i16,
    pub mm_share_auth_user: Option<String>,
    pub mm_share_auth_password: Option<String>,
    pub mm_network_share_workgroup: Option<String>,
}

pub async fn mk_lib_database_network_share_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBShareList>, sqlx::Error> {
    let table_rows: Vec<DBShareList> = sqlx::query_as(
        r#"select mm_network_share_guid, mm_network_share_ip, mm_network_share_path,
        mm_network_share_comment, mm_network_share_version,
        mm_share_auth_user,
        pgp_sym_decrypt(mm_share_auth_password::bytea, $1) AS mm_share_auth_password,
        mm_network_share_workgroup
        from mm_network_shares LEFT JOIN mm_share_auth
        ON mm_network_shares.mm_network_share_user_guid = mm_share_auth.mm_share_auth_guid
        order by mm_network_share_path"#,
    )
    .bind(share_auth_key())
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_network_share_delete(
    sqlx_pool: &sqlx::PgPool,
    network_share_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"delete from mm_network_shares where mm_network_share_guid = $1"#)
        .bind(network_share_uuid)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_network_share_insert(
    sqlx_pool: &sqlx::PgPool,
    network_share_ip: std::net::IpAddr,
    network_share_path: &str,
    network_share_comment: &str,
    network_share_version: i16
) -> Result<uuid::Uuid, sqlx::Error> {
    let share_path = parse_share_name(network_share_path).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "invalid share path (expected '//host/share' or '\\\\host\\share', got {network_share_path:?})"
        ))
    })?;
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_network_shares (mm_network_share_guid, mm_network_share_ip,
        mm_network_share_path, mm_network_share_comment, mm_network_share_version)
        values ($1, $2, $3, $4, $5)"#,
    )
    .bind(new_guid)
    .bind(network_share_ip)
    .bind(share_path)
    .bind(network_share_comment)
    .bind(network_share_version)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Point a share row at an existing `mm_share_auth` entry.
///
/// `network_share_auth_guid` must reference a row in `mm_share_auth`. To
/// rotate the stored credentials themselves, update `mm_share_auth` directly
/// (the password is encrypted with `pgp_sym_encrypt` there).
pub async fn mk_lib_database_network_share_update_user_info(
    sqlx_pool: &sqlx::PgPool,
    network_share_uuid: Uuid,
    network_share_auth_guid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_network_shares set mm_network_share_user_guid = $1
        where mm_network_share_guid = $2"#,
    )
    .bind(network_share_auth_guid)
    .bind(network_share_uuid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_share_name;

    #[test]
    fn backslash_unc_returns_share_name_without_host() {
        assert_eq!(
            parse_share_name(r"\\192.168.1.5\media").as_deref(),
            Some("media"),
        );
    }

    #[test]
    fn backslash_unc_with_subpath_drops_subpath() {
        assert_eq!(
            parse_share_name(r"\\host\media\movies\2020").as_deref(),
            Some("media"),
        );
    }

    #[test]
    fn forward_slash_unc_returns_share_name() {
        assert_eq!(
            parse_share_name("//192.168.1.5/media").as_deref(),
            Some("media"),
        );
    }

    #[test]
    fn forward_slash_unc_with_subpath_drops_subpath() {
        assert_eq!(
            parse_share_name("//host/media/movies").as_deref(),
            Some("media"),
        );
    }

    #[test]
    fn bare_share_name_is_returned_as_is() {
        assert_eq!(parse_share_name("media").as_deref(), Some("media"));
    }

    #[test]
    fn empty_string_returns_none() {
        assert!(parse_share_name("").is_none());
    }

    #[test]
    fn unc_without_share_segment_returns_none() {
        assert!(parse_share_name(r"\\host").is_none());
        assert!(parse_share_name("//host").is_none());
    }

    #[test]
    fn bare_ip_is_rejected() {
        assert!(parse_share_name("192.168.1.122").is_none());
        assert!(parse_share_name("10.0.0.1").is_none());
        assert!(parse_share_name("::1").is_none());
    }
}
