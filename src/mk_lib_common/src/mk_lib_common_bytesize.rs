// https://github.com/hyunsik/bytesize

use bytesize::ByteSize;

pub fn mk_lib_common_bits(bytes: u64, si: bool) -> String {
    let s = ByteSize(bytes.saturating_mul(8)).to_string_as(si);
    let without_byte_suffix = s.strip_suffix('B').unwrap_or(s.as_str());
    format!("{without_byte_suffix}bits")
}

pub fn mk_lib_common_bytesize(number_for_format: u64) -> Result<String, std::io::Error> {
    let result = ByteSize(number_for_format).to_string();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mk_lib_common_bits_zero() {
        assert_eq!(mk_lib_common_bits(0, false), "0bits");
    }

    #[test]
    fn test_mk_lib_common_bits_one_byte() {
        assert_eq!(mk_lib_common_bits(1, false), "8bits");
    }

    #[test]
    fn test_mk_lib_common_bits_1024_bytes() {
        assert_eq!(mk_lib_common_bits(1024, false), "8192bits");
    }

    #[test]
    fn test_mk_lib_common_bits_si() {
        let result = mk_lib_common_bits(1000, true);
        assert!(result.contains("bits"));
        assert!(!result.contains("B"));
    }

    #[test]
    fn test_mk_lib_common_bits_saturating_mul() {
        let result = mk_lib_common_bits(u64::MAX / 8 + 1, false);
        assert!(result.contains("bits"));
    }

    #[test]
    fn test_mk_lib_common_bytesize_zero() {
        assert_eq!(mk_lib_common_bytesize(0).unwrap(), "0 B");
    }

    #[test]
    fn test_mk_lib_common_bytesize_one_byte() {
        assert_eq!(mk_lib_common_bytesize(1).unwrap(), "1 B");
    }

    #[test]
    fn test_mk_lib_common_bytesize_1024() {
        assert_eq!(mk_lib_common_bytesize(1024).unwrap(), "1.0 KiB");
    }

    #[test]
    fn test_mk_lib_common_bytesize_1048576() {
        assert_eq!(mk_lib_common_bytesize(1048576).unwrap(), "1.0 MiB");
    }

    #[test]
    fn test_mk_lib_common_bytesize_large() {
        let result = mk_lib_common_bytesize(1_234_567_890).unwrap();
        assert!(result.contains("MiB") || result.contains("GiB"));
    }
}
