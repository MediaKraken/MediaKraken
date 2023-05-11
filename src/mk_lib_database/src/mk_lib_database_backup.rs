use chrono::prelude::*;

use serde::{Deserialize, Serialize};




use sqlx::{FromRow, Row};



#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBBackupList {
    pub mm_backup_guid: uuid::Uuid,
    pub mm_backup_description: String,
    pub mm_backup_location_type: i16,
    pub mm_backup_location: String,
    pub mm_backup_created: DateTime<Utc>,
}
