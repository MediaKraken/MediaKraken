// https://github.com/bcmyers/num-format

use num_format::{Locale, SystemLocale, ToFormattedString};
use std::env;

fn prefers_fallback_locale() -> bool {
    ["LC_ALL", "LC_NUMERIC", "LANG"]
        .into_iter()
        .filter_map(|key| env::var(key).ok())
        .map(|value| value.trim().to_ascii_uppercase())
        .any(|value| value == "C" || value.starts_with("C.") || value == "POSIX")
}

fn locale_from_name(locale_name: &str) -> Option<Locale> {
    let trimmed = locale_name.trim();
    if trimmed.is_empty() {
        return None;
    }

    Locale::from_name(trimmed)
        .ok()
        .or_else(|| Locale::from_name(trimmed.replace('-', "_")).ok())
        .or_else(|| {
            let mut parts = trimmed.split(['-', '_']);
            let language = parts.next()?;
            let region = parts.next()?;
            let normalized = format!(
                "{}_{}",
                language.to_ascii_lowercase(),
                region.to_ascii_uppercase()
            );
            Locale::from_name(&normalized).ok()
        })
}

pub fn mk_lib_common_internationalization_number_format(number_for_format: i64) -> String {
    if prefers_fallback_locale() {
        return number_for_format.to_formatted_string(&Locale::en);
    }

    match SystemLocale::default() {
        Ok(locale) => number_for_format.to_formatted_string(&locale),
        Err(_) => number_for_format.to_formatted_string(&Locale::en),
    }
}

pub fn mk_lib_common_internationalization_number_format_locale(
    number_for_format: i64,
    locale_name: Option<&str>,
) -> String {
    if let Some(locale) = locale_name.and_then(locale_from_name) {
        return number_for_format.to_formatted_string(&locale);
    }

    mk_lib_common_internationalization_number_format(number_for_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefers_fallback_locale_c() {
        std::env::set_var("LC_ALL", "C");
        assert!(prefers_fallback_locale());
        std::env::remove_var("LC_ALL");
    }

    #[test]
    fn test_prefers_fallback_locale_c_with_region() {
        std::env::set_var("LC_NUMERIC", "C_US");
        assert!(prefers_fallback_locale());
        std::env::remove_var("LC_NUMERIC");
    }

    #[test]
    fn test_prefers_fallback_locale_posix() {
        std::env::set_var("LANG", "POSIX");
        assert!(prefers_fallback_locale());
        std::env::remove_var("LANG");
    }

    #[test]
    fn test_prefers_fallback_locale_not_c() {
        std::env::set_var("LC_ALL", "en_US.UTF-8");
        assert!(!prefers_fallback_locale());
        std::env::remove_var("LC_ALL");
    }

    #[test]
    fn test_locale_from_name_empty() {
        assert!(locale_from_name("").is_none());
    }

    #[test]
    fn test_locale_from_name_whitespace() {
        assert!(locale_from_name("  ").is_none());
    }

    #[test]
    fn test_locale_from_name_en() {
        assert!(locale_from_name("en").is_some());
    }

    #[test]
    fn test_locale_from_name_german() {
        assert!(locale_from_name("de").is_some());
    }

    #[test]
    fn test_locale_from_name_with_hyphen() {
        assert!(locale_from_name("en-US").is_some());
    }

    #[test]
    fn test_locale_from_name_with_underscore() {
        assert!(locale_from_name("en_US").is_some());
    }

    #[test]
    fn test_locale_from_name_invalid() {
        assert!(locale_from_name("not_a_real_locale_xyz123").is_none());
    }

    #[test]
    fn test_number_format_returns_string() {
        let result = mk_lib_common_internationalization_number_format(1234567);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_number_format_negative() {
        let result = mk_lib_common_internationalization_number_format(-1234);
        assert!(result.contains('-'));
    }

    #[test]
    fn test_number_format_zero() {
        let result = mk_lib_common_internationalization_number_format(0);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_number_format_locale_with_name() {
        let result = mk_lib_common_internationalization_number_format_locale(1234567, Some("en"));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_number_format_locale_none() {
        let result = mk_lib_common_internationalization_number_format_locale(1234567, None);
        assert!(!result.is_empty());
    }
}
