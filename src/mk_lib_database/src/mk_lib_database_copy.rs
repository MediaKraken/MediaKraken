/*
CREATE TEMPORARY TABLE mktemp_import (
    temp_type, temp_key, temp_revision, temp_last_modified, temp_json
);
COPY mktemp_import FROM STDIN With CSV;

INSERT INTO mm_openlib_author(mm_openlib_author_id, mm_openlib_author_json)
SELECT temp_key, temp_json::jsonb
FROM mktemp_import7 ON conflict (mm_openlib_author_id)
DO update set mm_openlib_author_json=EXCLUDED.mm_openlib_author_json;

DROP TABLE mktemp_import7;
*/

/*
running
INSERT INTO mm_openlib_author(mm_openlib_author_id, mm_openlib_author_json)
SELECT temp_key, temp_json
FROM mktemp_import10 ON conflict (mm_openlib_author_id)
DO update set mm_openlib_author_json=EXCLUDED.mm_openlib_author_json;
 */

pub async fn mk_lib_database_copy(
    sqlx_pool: &sqlx::PgPool,
    copy_file: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mktemp_import10 (temp_type TEXT, temp_key TEXT, temp_revision TEXT, temp_last_modified TIMESTAMP, temp_json JSONB);",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    let mut pg_connection = sqlx_pool.acquire().await.unwrap();
    let mut pg_copy_in = pg_connection
        .copy_in_raw("copy public.mktemp_import10 (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin with delimiter E'\t' escape '\\' quote E'\x08' csv")
        .await
        .unwrap();
    let file = tokio::fs::File::open(copy_file).await?;
    pg_copy_in.read_from(file).await.unwrap();
    let rows_inserted = pg_copy_in.finish().await.unwrap();
    println!("New row inserted: {rows_inserted:}");
    Ok(())
}

// works on jsonb          .copy_in_raw("copy public.mktemp_import10 (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin with delimiter E'\t' escape '\\' quote E'\x08' csv")
// works but no quoates on json .copy_in_raw("copy public.mktemp_import (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin CSV DELIMITER E'\t' ESCAPE '\\'")
// works but no quotes on "json".copy_in_raw("copy public.mktemp_import (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin CSV DELIMITER E'\t' QUOTE '\"' ESCAPE '\\'")
// INSERT ... AS SELECT ... command, where the SELECT is doing conversion to JSONB
//         .copy_in_raw("copy public.mktemp_import3 (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin TEXT DELIMITER E'\t' QUOTE '\"' ESCAPE '\\'")
// ESCAPE '\'  - COPY escape must be a single one-byte character
// ESCAPE '\\' - First record fails - called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "22P02", message: "invalid input syntax for type json", detail: Some("Token \"type\" is invalid."), hint: None, position: None, where: Some("JSON data, line 1: {type...\nCOPY mktemp_import6, line 1, column temp_json: \"{type: {key: /type/author}, name: Eileen Schneemilch, key: /authors/OL10000979A, source_records: [bw...\""), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("jsonfuncs.c"), line: Some(627), routine: Some("json_ereport_error") })
// ESCAPE '\\\\' - COPY escape must be a single one-byte character
// QUOTE '\"' - removed - First record fails - called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "22P02", message: "invalid input syntax for type json", detail: Some("Token \"type\" is invalid."), hint: None, position: None, where: Some("JSON data, line 1: {type...\nCOPY mktemp_import6, line 1, column temp_json: \"{type: {key: /type/author}, name: Eileen Schneemilch, key: /authors/OL10000979A, source_records: [bw...\""), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("jsonfuncs.c"), line: Some(627), routine: Some("json_ereport_error") })