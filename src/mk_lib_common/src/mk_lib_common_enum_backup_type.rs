use lazy_static::lazy_static;
use std::collections::HashMap;

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
