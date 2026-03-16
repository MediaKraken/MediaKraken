use mk_lib_common;

pub mod filters {
    // pub fn as_u64<T: Into<u64>>(x: T) -> u64 {
    //     x.into()
    // }

    // pub fn t_as_i64<T: Into<i64>>(x: T) -> i64 {
    //     x.into()
    // }

    // pub fn t_as_u64<T>(value: T) where u64: From<T> {
    //    let b = u64::from(value);
    // }

    pub fn t_as_i64<T: std::fmt::Display>(s: T) -> i64 {
        let s = s.to_string();
        if let Ok(value) = s.parse::<i64>() {
            return value;
        }

        if let Ok(value) = s.parse::<f64>() {
            if value.is_finite() {
                return value.round() as i64;
            }
        }

        0
    }

    pub fn t_as_u64<T: std::fmt::Display>(s: T) -> u64 {
        let s = s.to_string();
        s.parse::<u64>().unwrap_or(0)
    }

    #[askama::filter_fn]
    pub fn space_to_html<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace(" ", "%20"))
    }

    #[askama::filter_fn]
    pub fn slash_to_asterik<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace("/", "*"))
    }

    #[askama::filter_fn]
    pub fn replace<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
        a: &str,
        b: &str,
    ) -> askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace(a, b))
    }
    // pub fn replace(s: &str, a: &str, b: &str) -> ::askama::Result<String> {
    //     Ok(s.replace(a, b))
    // }

    // #[askama::filter_fn]
    // pub fn uuid_to_str(s: &uuid::Uuid) -> ::askama::Result<String> {
    //     Ok(format!("{}", s))
    // }

    #[askama::filter_fn]
    pub fn number_format<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let result = mk_lib_common::mk_lib_common_internationalization::mk_lib_common_internationalization_number_format(t_as_i64(s));
        Ok(result)
    }

    #[askama::filter_fn]
    pub fn byte_format<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let result =
            mk_lib_common::mk_lib_common_bytesize::mk_lib_common_bytesize(t_as_u64(s)).unwrap();
        Ok(result)
    }

    #[askama::filter_fn]
    pub fn unquote_json<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace("\"", ""))
    }
}
