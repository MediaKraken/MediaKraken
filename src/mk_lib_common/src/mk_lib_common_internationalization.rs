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

pub fn mk_lib_common_internationalization_number_format(number_for_format: i64) -> String {
    if prefers_fallback_locale() {
        return number_for_format.to_formatted_string(&Locale::en);
    }

    match SystemLocale::default() {
        Ok(locale) => number_for_format.to_formatted_string(&locale),
        Err(_) => number_for_format.to_formatted_string(&Locale::en),
    }
}
