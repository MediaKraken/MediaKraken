/*
CREATE TEMPORARY TABLE mktemp_import (
    temp_type, temp_key, temp_revision, temp_last_modified, temp_json
);
COPY mktemp_import FROM STDIN With CSV;

INSERT INTO table_a(temp_id, temp_json)
SELECT *
FROM mktemp_import ON conflict (temp_id)
DO update set temp_json=EXCLUDED.temp_json;

DROP TABLE mktemp_import;
*/

pub async fn mk_lib_database_copy(
    sqlx_pool: &sqlx::PgPool,
    copy_file: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mktemp_import5 (temp_type TEXT, temp_key TEXT, temp_revision TEXT, temp_last_modified TIMESTAMP, temp_json TEXT);",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    let mut pg_connection = sqlx_pool.acquire().await.unwrap();
    let mut pg_copy_in = pg_connection
        .copy_in_raw("copy public.mktemp_import5 (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin (format text)")
        .await
        .unwrap();
    let file = tokio::fs::File::open(copy_file).await?;
    pg_copy_in.read_from(file).await.unwrap();
    let rows_inserted = pg_copy_in.finish().await.unwrap();
    println!("New row inserted: {rows_inserted:}");
    Ok(())
}
//         .copy_in_raw("copy public.mktemp_import3 (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin TEXT DELIMITER E'\t' QUOTE '\"' ESCAPE '\\'")
