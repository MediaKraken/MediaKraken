#[inline]
pub async fn mk_lib_filler() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_lib_filler_returns_ok() {
        assert!(mk_lib_filler().await.is_ok());
    }

    #[tokio::test]
    async fn test_mk_lib_filler_returns_unit() {
        let result = mk_lib_filler().await;
        assert!(result.is_ok());
    }
}
