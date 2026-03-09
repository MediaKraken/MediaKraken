use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::{Column, MySql, Pool, Postgres, QueryBuilder, Row, TypeInfo};

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
        env = "COLUMN_LIST",
        help = "Comma-separated list of columns to copy, in order (example: name,email,created_at)."
    )]
    columns: String,

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
    let columns = parse_columns(&cli.columns)?;

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

fn parse_columns(input: &str) -> Result<Vec<String>> {
    let cols = input
        .split(',')
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if cols.is_empty() {
        return Err(anyhow!("at least one column is required"));
    }

    Ok(cols)
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

    let result = sqlx::query("SELECT setval($1::regclass, $2::bigint, false)")
        .bind(sequence_name)
        .bind(1_000_000_000_i64)
        .execute(postgres)
        .await
        .context("failed to set sequence value")?;

    Ok(result)
}

async fn copy_rows(
    mysql: &Pool<MySql>,
    postgres: &Pool<Postgres>,
    mysql_table: &str,
    postgres_table: &str,
    columns: &[String],
    batch_size: usize,
) -> Result<u64> {
    if batch_size == 0 {
        return Err(anyhow!("batch_size must be greater than zero"));
    }

    let select_sql = format!(
        "SELECT {} FROM {}",
        columns
            .iter()
            .map(|c| format!("`{}`", c.replace('`', "``")))
            .collect::<Vec<_>>()
            .join(", "),
        mysql_table
    );

    let rows = sqlx::query(&select_sql)
        .fetch_all(mysql)
        .await
        .with_context(|| format!("failed to read data from MySQL table '{mysql_table}'"))?;

    let converted_rows = rows
        .iter()
        .map(|row| convert_row(row, columns.len()))
        .collect::<Result<Vec<_>>>()?;

    if converted_rows.is_empty() {
        return Ok(0);
    }

    let mut total_inserted = 0_u64;
    for chunk in converted_rows.chunks(batch_size) {
        let mut qb = QueryBuilder::<Postgres>::new(format!(
            "INSERT INTO {} ({}) ",
            postgres_table,
            columns
                .iter()
                .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        qb.push_values(chunk, |mut b, values| {
            for value in values {
                bind_sql_value(&mut b, value);
            }
        });

        let result = qb.build().execute(postgres).await.with_context(|| {
            format!("failed to insert batch into PostgreSQL table '{postgres_table}'")
        })?;

        total_inserted += result.rows_affected();
    }

    Ok(total_inserted)
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
