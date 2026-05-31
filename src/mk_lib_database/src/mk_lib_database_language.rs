use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLanguageList {
    pub id: i32,
    pub code: String,
    pub language: String,
}

pub async fn mk_lib_database_language_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBLanguageList>, sqlx::Error> {
    let table_rows: Vec<DBLanguageList> = sqlx::query_as(
        r#"select id, code, language
        from mm_languages
        order by language"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}