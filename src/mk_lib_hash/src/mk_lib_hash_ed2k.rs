// https://github.com/runfalk/ed2k-rs

pub fn mk_file_hash_ed2k(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let ed2k = Ed2k::from_path(file_to_read)?;
    return ed2k;
}
