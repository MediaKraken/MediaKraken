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
        "3": "S3"
    }"#).unwrap();
}

lazy_static! {
    pub static ref BACKUP_CLASS: Vec<(i32, String)> = vec![
        (0, "Amazon S3".to_string()),
        (1, "Local".to_string()),
        (2, "Network Share".to_string()),
        (3, "S3".to_string())];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_mutex_map_contains_all_keys() {
        assert_eq!(*BACKUP_MUTEX_COUNT, 4);
        assert!(BACKUP_MUTEX_MAP.contains_key(&0));
        assert!(BACKUP_MUTEX_MAP.contains_key(&1));
        assert!(BACKUP_MUTEX_MAP.contains_key(&2));
        assert!(BACKUP_MUTEX_MAP.contains_key(&3));
    }

    #[test]
    fn test_backup_mutex_map_values() {
        assert_eq!(BACKUP_MUTEX_MAP.get(&0).unwrap(), &"Amazon S3");
        assert_eq!(BACKUP_MUTEX_MAP.get(&1).unwrap(), &"Local");
        assert_eq!(BACKUP_MUTEX_MAP.get(&2).unwrap(), &"Network Share");
        assert_eq!(BACKUP_MUTEX_MAP.get(&3).unwrap(), &"S3");
    }

    #[test]
    fn test_backup_class_json() {
        assert_eq!(BACKUP_CLASS_JSON["0"], "Amazon S3");
        assert_eq!(BACKUP_CLASS_JSON["1"], "Local");
        assert_eq!(BACKUP_CLASS_JSON["2"], "Network Share");
        assert_eq!(BACKUP_CLASS_JSON["3"], "S3");
    }

    #[test]
    fn test_backup_class() {
        assert_eq!(BACKUP_CLASS.len(), 4);
        assert_eq!(BACKUP_CLASS[0], (0, "Amazon S3".to_string()));
        assert_eq!(BACKUP_CLASS[1], (1, "Local".to_string()));
        assert_eq!(BACKUP_CLASS[2], (2, "Network Share".to_string()));
        assert_eq!(BACKUP_CLASS[3], (3, "S3".to_string()));
    }

    #[test]
    fn test_backup_mutex_count_matches_map_len() {
        assert_eq!(*BACKUP_MUTEX_COUNT, BACKUP_MUTEX_MAP.len());
    }
}
