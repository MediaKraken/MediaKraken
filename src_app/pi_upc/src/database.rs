use sqlite::State;
use std::error::Error;

pub fn database_open() -> Result<sqlite::Connection, Box<dyn Error>> {
    let db = sqlite::open("upc_scan.db").unwrap();
    let query = "CREATE TABLE IF NOT EXISTS upc_codes \
            (upc_code INTEGER NOT NULL, \
            upc_code_type INTEGER NOT NULL, \
            added_timestamp DATETIME NOT NULL);";
    db.execute(query).unwrap();

    let query = "CREATE INDEX IF NOT EXISTS upc_code_ndx on upc_codes \
            (upc_code, upc_code_type);";
    db.execute(query).unwrap();

    let query = "CREATE TABLE IF NOT EXISTS logs \
            (log_timestamp DATETIME NOT NULL, \
            log_text TEXT)";
    db.execute(query).unwrap();

    let query = "CREATE INDEX IF NOT EXISTS log_time_ndx on logs \
            (log_timestamp);";
    db.execute(query).unwrap();

    Ok(db)
}

//pub fn database_close() -> Result<sqlx::Pool<Sqlite>, Box<dyn Error>> {}

pub fn database_insert_logs(
    db: &sqlite::Connection,
    log_text: String,
) -> Result<(), Box<dyn Error>> {
    let query = format!(
        "insert into logs (log_timestamp, log_text) \
        values (CURRENT_TIMESTAMP, '{}');",
        log_text,
    );
    db.execute(query).unwrap();
    Ok(())
}

pub fn database_upc_insert(
    db: &sqlite::Connection,
    upc_code: i64,
    upc_type: i32,
) -> Result<(), Box<dyn Error>> {
    let query = format!(
        "insert into upc_codes (upc_code, upc_code_type, added_timestamp) \
        values ({}, {}, CURRENT_TIMESTAMP);",
        upc_code, upc_type
    );
    db.execute(query).unwrap();
    Ok(())
}

pub fn database_upc_owned(
    db: &sqlite::Connection,
    upc_code: i64,
    upc_type: i32,
) -> Result<i64, Box<dyn Error>> {
    let mut record_count: i64 = 0;
    let query = format!(
        "SELECT count(*) as total_found FROM upc_codes WHERE upc_code = {} and upc_code_type = {}",
        upc_code, upc_type
    );
    let mut statement = db.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        record_count = statement.read::<i64, _>("total_found").unwrap();
    }
    Ok(record_count)
}

pub fn database_upc_known_count(
    db: &sqlite::Connection,
) -> Result<i64, Box<dyn Error>> {
    let mut record_count: i64 = 0;
    let query = "SELECT count(*) as total_found FROM upc_codes";
    let mut statement = db.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        record_count = statement.read::<i64, _>("total_found").unwrap();
    }
    Ok(record_count)
}
