// https://github.com/bcmyers/num-format

use num_format::SystemLocale;
use num_format::WriteFormatted;

pub fn mk_lib_common_internationalization_number_format(
    number_for_format: i64,
) -> Result<String, std::io::Error> {
    let locale = SystemLocale::default().unwrap();
    let mut writer = String::new();
    let _res = writer.write_formatted(&number_for_format, &locale);
    Ok(writer)
}
