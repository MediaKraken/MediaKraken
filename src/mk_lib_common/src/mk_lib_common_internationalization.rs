// https://github.com/bcmyers/num-format

use num_format::{Locale, SystemLocale, ToFormattedString};

pub fn mk_lib_common_internationalization_number_format(number_for_format: i64) -> String {
    match SystemLocale::default() {
        Ok(locale) => number_for_format.to_formatted_string(&locale),
        Err(_) => number_for_format.to_formatted_string(&Locale::en),
    }
}
