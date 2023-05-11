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
}
