use chrono::prelude::*;
use mk_lib_logging::mk_lib_logging;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use std::num::NonZeroU8;
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBBackupList {
    pub mm_backup_guid: uuid::Uuid,
    pub mm_backup_description: String,
    pub mm_backup_location_type: i16,
    pub mm_backup_location: String,
    pub mm_backup_created: DateTime<Utc>,
}
