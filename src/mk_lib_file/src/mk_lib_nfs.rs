use libnfs::*;
use std::error::Error;

/// Stub for future NFS share mounting.
///
/// The previous implementation ignored `share_to_mount` entirely and always
/// mounted the hard-coded `0.0.0.0:/srv/nfs`, which was actively dangerous:
/// it could attach to any NFS server reachable from the process. Until a
/// real implementation lands we return an error so callers surface the gap
/// instead of silently mounting the wrong target.
pub fn mk_file_nfs_client_connect(
    _share_to_mount: mk_lib_database::mk_lib_database_network_share::DBShareList,
) -> Result<NFSClient, Box<dyn Error>> {
    Err("mk_file_nfs_client_connect is not implemented".into())
}

/*
 let dir = nfs.opendir(&Path::new("/"))?;
    for f in dir {
        println!("dir: {:?}", f);
    }

    println!("creating file");
    let file = nfs.create(
        &Path::new("/rust"),
        OFlag::O_SYNC,
        Mode::S_IROTH | Mode::S_IWOTH,
    )?;
    let mut contents = String::from("Hello from rust").into_bytes();
    file.write(&mut contents)?;

    println!("reading file");
    let file = nfs.open(&Path::new("/rust"), OFlag::O_RDONLY)?;
    let buff = file.read(1024)?;
    println!("read file: {}", String::from_utf8_lossy(&buff));
     */
