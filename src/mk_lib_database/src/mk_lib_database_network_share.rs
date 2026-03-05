use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

pub async fn mk_lib_database_network_share_exists(
    sqlx_pool: &sqlx::PgPool,
    network_share_ip: std::net::IpAddr,
    network_share_path: serde_json::Value,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_network_shares where mm_network_share_ip = $1 and mm_network_share_path = $2 limit 1) as found_record limit 1"#,
    )
    .bind(network_share_ip)
    .bind(network_share_path)
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
        r#"select mm_share_auth_guid, mm_share_auth_user from mm_share_auth order by mm_share_auth_user"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBShareList {
    pub mm_network_share_guid: uuid::Uuid,
    pub mm_network_share_ip: std::net::IpAddr,
    pub mm_network_share_path: String,
    pub mm_network_share_comment: String,
    pub mm_share_auth_user: String,
    pub mm_share_auth_password: Option<String>,
    pub mm_network_share_version: Option<bool>,
    pub mm_network_share_workgroup: Option<String>,
}

pub async fn mk_lib_database_network_share_detail(
    sqlx_pool: &sqlx::PgPool,
    share_guid: uuid::Uuid,
) -> Result<DBShareList, sqlx::Error> {
    let table_row: DBShareList = sqlx::query_as(
        r#"select mm_network_share_guid, mm_network_share_ip, mm_network_share_path, mm_network_share_comment, mm_share_auth_user, mm_share_auth_password, mm_network_share_version, mm_network_share_workgroup from mm_network_shares, mm_share_auth where mm_network_share_user_guid = mm_share_auth_guid and mm_network_share_guid = $1"#,
    )
    .bind(share_guid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(table_row)
}

pub async fn mk_lib_database_network_share_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBShareList>, sqlx::Error> {
    let table_rows: Vec<DBShareList> = sqlx::query_as(
        r#"select mm_network_share_guid, mm_network_share_ip, mm_network_share_path, mm_network_share_comment, mm_share_auth_user, mm_share_auth_password, mm_network_share_version, mm_network_share_workgroup from mm_network_shares, mm_share_auth where mm_network_share_user_guid = mm_share_auth_guid order by mm_network_share_path"#,
    )
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
    network_share_comment: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    let path_name = network_share_path.replace("\\\\", "/");
    let path_vec: Vec<&str> = path_name.splitn(3, '/').collect();
    sqlx::query(
        r#"insert into mm_network_shares (mm_network_share_guid, mm_network_share_ip, mm_network_share_path, mm_network_share_comment) values ($1, $2, $3, $4)"#,
    )
    .bind(new_guid)
    .bind(network_share_ip)
    .bind(path_vec[1])
    .bind(network_share_comment.as_str())
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_network_share_update_user_info(
    sqlx_pool: &sqlx::PgPool,
    network_share_uuid: Uuid,
    network_share_user: String,
    network_share_password: String,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"udpate mm_network_shares set mm_network_share_user = $1, mm_network_share_password = $2 where mm_network_share_guid = $3"#,
    )
    .bind(network_share_user)
    .bind(network_share_password)
    .bind(network_share_uuid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
