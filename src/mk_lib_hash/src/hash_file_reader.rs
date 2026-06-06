use std::io;
use tokio::{fs::File, io::AsyncReadExt};

const HASH_BUFFER_SIZE: usize = 64 * 1024;

pub async fn read_file_chunks(
    file_to_read: &str,
    mut on_chunk: impl FnMut(&[u8]),
) -> io::Result<()> {
    let mut file = File::open(file_to_read).await?;
    let mut buffer = [0_u8; HASH_BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        on_chunk(&buffer[..bytes_read]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_file_chunks_existing_file() {
        let mut chunks = Vec::new();
        read_file_chunks("testing_data/HashCalc.txt", |chunk| {
            chunks.push(chunk.to_vec());
        })
        .await
        .unwrap();
        assert!(!chunks.is_empty());
    }

    #[tokio::test]
    async fn test_read_file_chunks_nonexistent_file() {
        let result = read_file_chunks("nonexistent_file.txt", |_| {}).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_read_file_chunks_accumulates_all_bytes() {
        let mut all_bytes = Vec::new();
        read_file_chunks("testing_data/HashCalc.txt", |chunk| {
            all_bytes.extend_from_slice(chunk);
        })
        .await
        .unwrap();
        let content = std::fs::read_to_string("testing_data/HashCalc.txt").unwrap();
        assert_eq!(all_bytes.len(), content.len());
        assert_eq!(all_bytes, content.as_bytes());
    }

    #[test]
    fn test_hash_buffer_size_is_64k() {
        assert_eq!(HASH_BUFFER_SIZE, 64 * 1024);
    }
}
