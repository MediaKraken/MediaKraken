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
        .or_else(|| Locale::from_name(&trimmed.replace('-', "_")).ok())
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
