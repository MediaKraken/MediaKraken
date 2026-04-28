use mk_lib_common;

pub mod filters {
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
    pub fn url_encode<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        Ok(urlencoding::encode(&s.to_string()).into_owned())
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

    #[askama::filter_fn]
    pub fn number_format<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let result = mk_lib_common::mk_lib_common_internationalization::mk_lib_common_internationalization_number_format(t_as_i64(s));
        Ok(result)
    }

    #[askama::filter_fn]
    pub fn number_format_locale<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
        locale_name: &str,
    ) -> askama::Result<String> {
        let result = mk_lib_common::mk_lib_common_internationalization::
            mk_lib_common_internationalization_number_format_locale(t_as_i64(s), Some(locale_name));
        Ok(result)
    }

    #[askama::filter_fn]
    pub fn byte_format<T: std::fmt::Display>(
        s: T,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        let raw = s.to_string();
        let value = raw.parse::<u64>().unwrap_or(0);
        match mk_lib_common::mk_lib_common_bytesize::mk_lib_common_bytesize(value) {
            Ok(result) => Ok(result),
            Err(error) => {
                tracing::warn!(?error, raw = %raw, "byte_format filter failed; returning raw value");
                Ok(raw)
            }
        }
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
