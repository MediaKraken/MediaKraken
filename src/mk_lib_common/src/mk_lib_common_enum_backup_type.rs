use lazy_static::lazy_static;
use std::collections::HashMap;
use serde_json::Value;

// BACKUP_MUTEX_MAP.get(&0).unwrap()

lazy_static! {
    pub static ref BACKUP_MUTEX_MAP: HashMap<u8, &'static str> = {
        let mut backup_type = HashMap::new();
        backup_type.insert(0, "Amazon S3");
        backup_type.insert(1, "Local");
        backup_type.insert(2, "Network Share");
        backup_type.insert(3, "S3");
        backup_type
    };
    pub static ref BACKUP_MUTEX_COUNT: usize = BACKUP_MUTEX_MAP.len();
}

lazy_static! {
    pub static ref BACKUP_CLASS_JSON: Value = serde_json::from_str(r#"
    {
        "0": "Amazon S3",
        "1": "Local",
        "2": "Network Share",
        "3": "S3",
    }"#).unwrap();
}

lazy_static! {
    pub static ref BACKUP_CLASS: Vec<(i32, String)> = vec![
        (0, "Amazon S3".to_string()),
        (1, "Local".to_string()),
        (2, "Network Share".to_string()),
        (3, "S3".to_string())];
}
