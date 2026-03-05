/*
CREATE TEMPORARY TABLE mktemp_import (
    temp_type, temp_key, temp_revision, temp_last_modified, temp_json
);
COPY mktemp_import FROM STDIN With CSV;

INSERT INTO mm_openlib_author(mm_openlib_author_id, mm_openlib_author_json)
SELECT temp_key, temp_json::jsonb
FROM mktemp_import7 ON conflict (mm_openlib_author_id)
DO update set mm_openlib_author_json=EXCLUDED.mm_openlib_author_json;

DROP TABLE mktemp_import;
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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = sqlx_pool.acquire().await?;
    sqlx::query(
        r#"CREATE TEMPORARY TABLE mktemp_import (
            temp_type TEXT, 
            temp_key TEXT, 
            temp_revision TEXT, 
            temp_last_modified TIMESTAMP, 
            temp_json JSONB
        ) ON COMMIT DROP;"#,
    )
    .execute(&mut *conn)
    .await?;
    let mut pg_copy_in = conn
        .copy_in_raw("COPY mktemp_import (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) FROM STDIN WITH DELIMITER E'\t' ESCAPE '\\' QUOTE E'\x08' CSV")
        .await?;
    let file = tokio::fs::File::open(copy_file).await?;
    pg_copy_in.read_from(file).await?;
    let rows_inserted = pg_copy_in.finish().await?;
    println!("Successfully streamed {rows_inserted} rows into mktemp_import for {copy_file}");
    Ok(())
}

pub async fn mk_lib_database_copy_author_upsert(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO mm_openlib_author (mm_openlib_author_id, mm_openlib_author_json)
        SELECT temp_key, temp_json
        FROM mktemp_import
        ON CONFLICT (mm_openlib_author_id)
        DO UPDATE SET mm_openlib_author_json = EXCLUDED.mm_openlib_author_json
        WHERE mm_openlib_author.mm_openlib_author_json IS DISTINCT FROM EXCLUDED.mm_openlib_author_json;"#,
    )
    .execute(&mut *transaction)
    .await?;
    sqlx::query(r#"truncate mktemp_import;"#)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_copy_edition_upsert(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO mm_openlib_edition (mm_openlib_edition_id, mm_openlib_edition_json)
        SELECT temp_key, temp_json
        FROM mktemp_import
        ON CONFLICT (mm_openlib_edition_id)
        DO UPDATE SET mm_openlib_edition_json = EXCLUDED.mm_openlib_edition_json
        WHERE mm_openlib_edition.mm_openlib_edition_json IS DISTINCT FROM EXCLUDED.mm_openlib_edition_json;"#,
    )
    .execute(&mut *transaction)
    .await?;
    sqlx::query(r#"truncate mktemp_import;"#)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_copy_work_upsert(sqlx_pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO mm_openlib_work (mm_openlib_work_id, mm_openlib_work_json)
        SELECT temp_key, temp_json
        FROM mktemp_import
        ON CONFLICT (mm_openlib_work_id)
        DO UPDATE SET mm_openlib_work_json = EXCLUDED.mm_openlib_work_json
        WHERE mm_openlib_work.mm_openlib_work_json IS DISTINCT FROM EXCLUDED.mm_openlib_work_json;"#,
    )
    .execute(&mut *transaction)
    .await?;
    sqlx::query(r#"truncate mktemp_import;"#)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
