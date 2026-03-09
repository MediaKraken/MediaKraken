use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use futures_util::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::{Column, MySql, Pool, Postgres, QueryBuilder, Row, TypeInfo};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(
    name = "test_mysql_to_postgres_sqlx",
    about = "Copy rows from a MySQL 8 table into an existing PostgreSQL 18 table with BIGSERIAL sequence reset to 1,000,000,000 first."
)]
struct Cli {
    #[arg(long, env = "MYSQL_URL")]
    mysql_url: String,

    #[arg(long, env = "POSTGRES_URL")]
    postgres_url: String,

    #[arg(long, env = "MYSQL_TABLE")]
    mysql_table: String,

    #[arg(long, env = "POSTGRES_TABLE")]
    postgres_table: String,

    #[arg(
        long,
        env = "SERIAL_COLUMN",
        default_value = "id",
        help = "BIGSERIAL column name in PostgreSQL table."
    )]
    serial_column: String,

    #[arg(
        long,
        env = "BATCH_SIZE",
        default_value_t = 1000,
        help = "Rows per insert batch."
    )]
    batch_size: usize,
}

#[derive(Debug, Clone)]
struct ColumnPair {
    mysql_column: String,
    postgres_column: String,
}

#[derive(Debug, Clone)]
enum SqlValue {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    Json(serde_json::Value),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mysql = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&cli.mysql_url)
        .await
        .context("failed to connect to MySQL")?;

    let postgres = PgPoolOptions::new()
        .max_connections(5)
        .connect(&cli.postgres_url)
        .await
        .context("failed to connect to PostgreSQL")?;

    let columns = discover_columns(
        &mysql,
        &postgres,
        &cli.mysql_table,
        &cli.postgres_table,
        &cli.serial_column,
    )
    .await?;

    set_bigserial_start(&postgres, &cli.postgres_table, &cli.serial_column).await?;
    println!(
        "Sequence for {}.{} set to 1,000,000,000.",
        cli.postgres_table, cli.serial_column
    );

    let copied = copy_rows(
        &mysql,
        &postgres,
        &cli.mysql_table,
        &cli.postgres_table,
        &columns,
        cli.batch_size,
    )
    .await?;

    println!("Done. Copied {copied} row(s).");
    Ok(())
}

async fn set_bigserial_start(
    postgres: &Pool<Postgres>,
    table: &str,
    serial_column: &str,
) -> Result<PgQueryResult> {
    let sequence_name: String = sqlx::query_scalar("SELECT pg_get_serial_sequence($1, $2)")
        .bind(table)
        .bind(serial_column)
        .fetch_one(postgres)
        .await
        .with_context(|| {
            format!(
                "failed to resolve sequence name for table '{table}' and column '{serial_column}'"
            )
        })?;

    sqlx::query("SELECT setval($1::regclass, $2::bigint, false)")
        .bind(sequence_name)
        .bind(1_000_000_000_i64)
        .execute(postgres)
        .await
        .context("failed to set sequence value")
}

async fn discover_columns(
    mysql: &Pool<MySql>,
    postgres: &Pool<Postgres>,
    mysql_table_input: &str,
    postgres_table_input: &str,
    serial_column: &str,
) -> Result<Vec<ColumnPair>> {
    let (mysql_schema_opt, mysql_table) = parse_table_name(mysql_table_input)?;
    let mysql_schema = match mysql_schema_opt {
        Some(schema) => schema,
        None => sqlx::query_scalar::<_, String>("SELECT DATABASE()")
            .fetch_one(mysql)
            .await
            .context("failed to resolve current MySQL database")?,
    };

    let (postgres_schema_opt, postgres_table) = parse_table_name(postgres_table_input)?;
    let postgres_schema = postgres_schema_opt.unwrap_or_else(|| "public".to_string());

    let mysql_columns: Vec<String> = sqlx::query_scalar(
        "SELECT column_name
         FROM information_schema.columns
         WHERE table_schema = ? AND table_name = ?
         ORDER BY ordinal_position",
    )
    .bind(&mysql_schema)
    .bind(&mysql_table)
    .fetch_all(mysql)
    .await
    .with_context(|| format!("failed to load columns for MySQL table '{mysql_table_input}'"))?;

    let postgres_columns: Vec<String> = sqlx::query_scalar(
        "SELECT column_name
         FROM information_schema.columns
         WHERE table_schema = $1 AND table_name = $2
         ORDER BY ordinal_position",
    )
    .bind(&postgres_schema)
    .bind(&postgres_table)
    .fetch_all(postgres)
    .await
    .with_context(|| {
        format!("failed to load columns for PostgreSQL table '{postgres_table_input}'")
    })?;

    let mysql_lookup: HashMap<String, String> = mysql_columns
        .into_iter()
        .map(|c| (c.to_ascii_lowercase(), c))
        .collect();

    let serial_lower = serial_column.to_ascii_lowercase();
    let column_pairs = postgres_columns
        .into_iter()
        .filter(|pg_col| pg_col.to_ascii_lowercase() != serial_lower)
        .filter_map(|pg_col| {
            mysql_lookup
                .get(&pg_col.to_ascii_lowercase())
                .map(|mysql_col| ColumnPair {
                    mysql_column: mysql_col.clone(),
                    postgres_column: pg_col,
                })
        })
        .collect::<Vec<_>>();

    if column_pairs.is_empty() {
        return Err(anyhow!(
            "no shared non-serial columns found between MySQL table '{}' and PostgreSQL table '{}'",
            mysql_table_input,
            postgres_table_input
        ));
    }

    Ok(column_pairs)
}

fn parse_table_name(input: &str) -> Result<(Option<String>, String)> {
    let parts: Vec<&str> = input.split('.').collect();
    match parts.as_slice() {
        [table] => Ok((None, sanitize_identifier(table)?)),
        [schema, table] => Ok((
            Some(sanitize_identifier(schema)?),
            sanitize_identifier(table)?,
        )),
        _ => Err(anyhow!(
            "invalid table identifier '{}'; expected table or schema.table",
            input
        )),
    }
}

fn sanitize_identifier(identifier: &str) -> Result<String> {
    let valid = !identifier.is_empty()
        && identifier
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_');

    if !valid {
        return Err(anyhow!(
            "invalid identifier '{}': only [A-Za-z0-9_] is allowed",
            identifier
        ));
    }

    Ok(identifier.to_string())
}

async fn copy_rows(
    mysql: &Pool<MySql>,
    postgres: &Pool<Postgres>,
    mysql_table: &str,
    postgres_table: &str,
    columns: &[ColumnPair],
    batch_size: usize,
) -> Result<u64> {
    if batch_size == 0 {
        return Err(anyhow!("batch_size must be greater than zero"));
    }

    let select_sql = format!(
        "SELECT {} FROM {}",
        columns
            .iter()
            .map(|c| format!("`{}`", c.mysql_column.replace('`', "``")))
            .collect::<Vec<_>>()
            .join(", "),
        mysql_table
    );

    let mut stream = sqlx::query(&select_sql).fetch(mysql);

    let mut total_inserted = 0_u64;
    let mut pending = Vec::with_capacity(batch_size);

    while let Some(row) = stream
        .try_next()
        .await
        .with_context(|| format!("failed to read data from MySQL table '{mysql_table}'"))?
    {
        pending.push(convert_row(&row, columns.len())?);

        if pending.len() >= batch_size {
            total_inserted += flush_batch(postgres, postgres_table, columns, &pending).await?;
            pending.clear();
        }
    }

    if !pending.is_empty() {
        total_inserted += flush_batch(postgres, postgres_table, columns, &pending).await?;
    }

    Ok(total_inserted)
}

async fn flush_batch(
    postgres: &Pool<Postgres>,
    postgres_table: &str,
    columns: &[ColumnPair],
    rows: &[Vec<SqlValue>],
) -> Result<u64> {
    let mut qb = QueryBuilder::<Postgres>::new(format!(
        "INSERT INTO {} ({}) ",
        postgres_table,
        columns
            .iter()
            .map(|c| format!("\"{}\"", c.postgres_column.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    qb.push_values(rows, |mut b, values| {
        for value in values {
            bind_sql_value(&mut b, value);
        }
    });

    qb.build()
        .execute(postgres)
        .await
        .map(|result| result.rows_affected())
        .with_context(|| format!("failed to insert batch into PostgreSQL table '{postgres_table}'"))
}

fn convert_row(row: &sqlx::mysql::MySqlRow, column_count: usize) -> Result<Vec<SqlValue>> {
    let mut converted = Vec::with_capacity(column_count);

    for idx in 0..column_count {
        let type_name = row.columns()[idx].type_info().name().to_ascii_lowercase();
        let value = match type_name.as_str() {
            "tinyint" if is_bool_like(row, idx) => row
                .try_get::<Option<bool>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::Bool),
            "tinyint" | "smallint" | "mediumint" | "int" | "bigint" => row
                .try_get::<Option<i64>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::I64),
            "float" | "double" | "real" => row
                .try_get::<Option<f64>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::F64),
            "decimal" | "numeric" => row
                .try_get::<Option<String>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::String),
            "date" => row
                .try_get::<Option<NaiveDate>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::Date),
            "time" => row
                .try_get::<Option<NaiveTime>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::Time),
            "datetime" | "timestamp" => row
                .try_get::<Option<NaiveDateTime>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::DateTime),
            "json" => row
                .try_get::<Option<serde_json::Value>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::Json),
            "blob" | "tinyblob" | "mediumblob" | "longblob" | "binary" | "varbinary" => row
                .try_get::<Option<Vec<u8>>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::Bytes),
            _ => row
                .try_get::<Option<String>, _>(idx)?
                .map_or(SqlValue::Null, SqlValue::String),
        };

        converted.push(value);
    }

    Ok(converted)
}

fn is_bool_like(row: &sqlx::mysql::MySqlRow, idx: usize) -> bool {
    row.columns()[idx].name().eq_ignore_ascii_case("is_active")
        || row.columns()[idx].name().starts_with("is_")
}

fn bind_sql_value(
    builder: &mut sqlx::query_builder::Separated<'_, '_, Postgres, &'static str>,
    value: &SqlValue,
) {
    match value {
        SqlValue::Null => {
            builder.push_bind(Option::<String>::None);
        }
        SqlValue::Bool(v) => {
            builder.push_bind(*v);
        }
        SqlValue::I64(v) => {
            builder.push_bind(*v);
        }
        SqlValue::F64(v) => {
            builder.push_bind(*v);
        }
        SqlValue::String(v) => {
            builder.push_bind(v);
        }
        SqlValue::Bytes(v) => {
            builder.push_bind(v);
        }
        SqlValue::Date(v) => {
            builder.push_bind(*v);
        }
        SqlValue::Time(v) => {
            builder.push_bind(*v);
        }
        SqlValue::DateTime(v) => {
            builder.push_bind(*v);
        }
        SqlValue::Json(v) => {
            builder.push_bind(sqlx::types::Json(v));
        }
    }
}
