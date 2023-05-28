// https://forum.opensubtitles.org/viewtopic.php?f=8&t=14563
// https://opensubtitles.stoplight.io/docs/opensubtitles-api/e3750fd63a100-getting-started

use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;

const HASH_BLK_SIZE: u64 = 65536;

pub async fn provider_opensubtitles_create_hash(
    filename: String,
) -> Result<String, std::io::Error> {
    let fsize = fs::metadata(filename.clone()).unwrap().len();
    if fsize > HASH_BLK_SIZE {
        let file = File::open(filename).unwrap();
        let mut buf = [0u8; 8];
        let mut word: u64;
        let mut hash_val: u64 = fsize; // seed hash with file size
        let iterations = HASH_BLK_SIZE / 8;
        let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);
        for _ in 0..iterations {
            reader.read(&mut buf)?;
            unsafe {
                word = mem::transmute(buf);
            };
            hash_val = hash_val.wrapping_add(word);
        }
        reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE))?;
        for _ in 0..iterations {
            reader.read(&mut buf)?;
            unsafe {
                word = mem::transmute(buf);
            };
            hash_val = hash_val.wrapping_add(word);
        }
        let hash_string = format!("{:01$x}", hash_val, 16);
        Ok(hash_string)
    } else {
        Ok("".to_string())
    }
}
