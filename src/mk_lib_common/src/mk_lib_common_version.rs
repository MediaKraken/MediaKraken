pub static MOBILE_ANDROID_VERSION: &str = "0.0.1";
pub static MOBILE_IOS_VERSION: &str = "0.0.1";
pub static THEATER_VERSION: &str = "0.0.1";
pub static WEB_VERSION: &str = "0.0.1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_android_version() {
        assert_eq!(MOBILE_ANDROID_VERSION, "0.0.1");
    }

    #[test]
    fn test_mobile_ios_version() {
        assert_eq!(MOBILE_IOS_VERSION, "0.0.1");
    }

    #[test]
    fn test_theater_version() {
        assert_eq!(THEATER_VERSION, "0.0.1");
    }

    #[test]
    fn test_web_version() {
        assert_eq!(WEB_VERSION, "0.0.1");
    }

    #[test]
    fn test_all_versions_non_empty() {
        assert!(!MOBILE_ANDROID_VERSION.is_empty());
        assert!(!MOBILE_IOS_VERSION.is_empty());
        assert!(!THEATER_VERSION.is_empty());
        assert!(!WEB_VERSION.is_empty());
    }
}
