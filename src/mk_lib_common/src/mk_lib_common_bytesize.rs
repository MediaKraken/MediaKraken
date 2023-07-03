// https://github.com/hyunsik/bytesize

use bytesize::ByteSize;

pub fn mk_lib_common_bits(bytes: u64, si: bool) -> String {
    let mut s = ByteSize(bytes * 8).to_string_as(si);
    s.pop();
    format!("{}bits", s)
}

pub fn mk_lib_common_bytesize(
    number_for_format: u64,
) -> Result<String, std::io::Error> {
    let result = ByteSize(number_for_format).to_string();
    Ok(result)
}
