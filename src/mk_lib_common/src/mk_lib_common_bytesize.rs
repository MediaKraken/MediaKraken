// https://github.com/hyunsik/bytesize

use bytesize::ByteSize;

pub fn mk_lib_common_bits(bytes: u64, si: bool) -> String {
    let s = ByteSize(bytes.saturating_mul(8)).to_string_as(si);
    let without_byte_suffix = s.strip_suffix('B').unwrap_or(s.as_str());
    format!("{without_byte_suffix}bits")
}

pub fn mk_lib_common_bytesize(
    number_for_format: u64,
) -> Result<String, std::io::Error> {
    let result = ByteSize(number_for_format).to_string();
    Ok(result)
}
