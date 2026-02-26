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
