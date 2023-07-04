use mk_lib_common;

pub mod filters {
    pub fn space_to_html(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(" ", "%20"))
    }

    pub fn slash_to_asterik(s: &str) -> ::askama::Result<String> {
        Ok(s.replace("/", "*"))
    }

    pub fn replace(s: &str, a: &str, b: &str) -> ::askama::Result<String> {
        Ok(s.replace(a, b))
    }

    pub fn uuid_to_str(s: &uuid::Uuid) -> ::askama::Result<String> {
        Ok(format!("{}", s))
    }

    pub fn number_format(s: &f32) -> ::askama::Result<String> {
        let result = mk_lib_common::mk_lib_common_internationalization::mk_lib_common_internationalization_number_format(s.clone() as i64).unwrap();
        Ok(result)
    }

    pub fn byte_format(s: &i64) -> ::askama::Result<String> {
        let result = mk_lib_common::mk_lib_common_bytesize::mk_lib_common_bytesize(s.clone() as u64).unwrap();
        Ok(result)
    }
}

