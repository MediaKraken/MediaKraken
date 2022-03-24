use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_link_delete(pool: &sqlx::PgPool,
                                         link_uuid: uuid::Uuid)
                                         -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_link where mm_link_guid = $1")
        .bind(link_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLinkList {
	mm_link_guid: uuid::Uuid,
	mm_link_name: String,
	mm_link_json: Json,
}

pub async fn mk_lib_database_link_list(pool: &sqlx::PgPool,
                                       offset: i32,
                                       records: i32)
                                       -> Result<Vec<DBLinkList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_link_guid, mm_link_name, \
        mm_link_json from mm_link \
        order by mm_link_name \
        offset $1 limit $2")
        .bind(offset)
        .bind(records);
    let table_rows: Vec<DBLinkList> = select_query
		.map(|row: PgRow| DBLinkList {
			mm_link_guid: row.get("mm_link_guid"),
			mm_link_name: row.get("mm_link_name"),
			mm_link_json: row.get("mm_link_json"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_link_insert(pool: &sqlx::PgPool,
                                         link_json: serde_json::Value)
                                         -> Result<(uuid::Uuid), sqlx::Error> {
    new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_link (mm_link_guid, mm_link_json) \
        values ($1, $2)")
        .bind(new_guid)
        .bind(link_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_link_list_count(pool: &sqlx::PgPool,
                                             search_value: String)
                                             -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_library_link \
            where mm_link_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_library_link")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}