use libnfs::*;
use nix::{fcntl::OFlag, sys::stat::Mode};

pub fn mk_file_nfs_client_connect(
    share_to_mount: mk_lib_database::mk_lib_database_network_share::DBShareList,
) -> Result<NFSClient, Box<dyn Error>> {
    let mut nfs = Nfs::new()?;
    nfs.set_uid(1000)?;
    nfs.set_gid(1000)?;
    nfs.set_debug(9)?;
    nfs.mount("0.0.0.0", "/srv/nfs")?;
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