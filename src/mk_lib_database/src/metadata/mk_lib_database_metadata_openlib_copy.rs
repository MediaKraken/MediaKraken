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
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mktemp_import (temp_type TEXT, temp_key TEXT, temp_revision TEXT, temp_last_modified TIMESTAMP, temp_json JSONB);",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    let mut pg_connection = sqlx_pool.acquire().await.unwrap();
    let mut pg_copy_in = pg_connection
        .copy_in_raw("copy public.mktemp_import (temp_type, temp_key, temp_revision, temp_last_modified, temp_json) from stdin with delimiter E'\t' escape '\\' quote E'\x08' csv")
        .await
        .unwrap();
    let file = tokio::fs::File::open(copy_file).await?;
    pg_copy_in.read_from(file).await.unwrap();
    let rows_inserted = pg_copy_in.finish().await.unwrap();
    println!("New row inserted: {rows_inserted:}");
    Ok(())
}

pub async fn mk_lib_database_copy_author_upsert(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "INSERT INTO mm_openlib_author(mm_openlib_author_id, mm_openlib_author_json)
        SELECT temp_key, temp_json
        FROM mktemp_import ON conflict (mm_openlib_author_id)
        DO update set mm_openlib_author_json=EXCLUDED.mm_openlib_author_json;",
    )
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
        "INSERT INTO mm_openlib_edition(mm_openlib_edition_id, mm_openlib_edition_json)
        SELECT temp_key, temp_json
        FROM mktemp_import ON conflict (mm_openlib_edition_id)
        DO update set mm_openlib_edition_json=EXCLUDED.mm_openlib_edition_json;",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_copy_rating_upsert(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "INSERT INTO mm_openlib_rating(mm_openlib_rating_id, mm_openlib_rating_json)
        SELECT temp_key, temp_json
        FROM mktemp_import ON conflict (mm_openlib_rating_id)
        DO update set mm_openlib_rating_json=EXCLUDED.mm_openlib_rating_json;",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_copy_work_upsert(
    sqlx_pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "INSERT INTO mm_openlib_work(mm_openlib_work_id, mm_openlib_work_json)
        SELECT temp_key, temp_json
        FROM mktemp_import ON conflict (mm_openlib_work_id)
        DO update set mm_openlib_work_json=EXCLUDED.mm_openlib_work_json;",
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}