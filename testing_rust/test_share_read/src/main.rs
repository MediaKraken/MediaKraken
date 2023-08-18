use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions, SmbStat};
use std::path::PathBuf;

fn main() {
    // setup server
    let client = SmbClient::new(
        SmbCredentials::default()
            .server("smb://192.168.1.187")
            .share("/daworks")
            .username("gfdgdg")
            .password("gfgdfgfd!")
            .workgroup("WORKGROUP"),
        SmbOptions::default().one_share_per_server(true),
    )
    .unwrap();
    let stats = client.stat("/Media");
    match stats {
        Ok(file_stat) => {
            println!("{:?}", file_stat.modified);
        }
        Err(_) => {
            println!("boom");
        }
    };
    //println!("stats: {:?}", stats);
    //tree(&client, "/", 0);
}

fn tree(client: &SmbClient, uri: &str, depth: usize) {
    // scan dir
    for entity in client.list_dir(uri).unwrap().into_iter() {
        // stat file
        let entity_uri = entity_uri(&entity, uri);
        let stat = client.stat(entity_uri.as_str()).unwrap();
        print_entry(&entity, &stat, depth);
        // if is dir, iter directory
        if entity.get_type() == SmbDirentType::Dir {
            tree(client, entity_uri.as_str(), depth + 1);
        }
    }
}

fn entity_uri(entity: &SmbDirent, path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.push(PathBuf::from(entity.name()));
    p.as_path().to_string_lossy().to_string()
}

fn print_entry(entity: &SmbDirent, stat: &SmbStat, depth: usize) {
    println!(
        "{}{:32}\t{}\t{}\t{}",
        fmt_depth(depth),
        entity.name(),
        stat.uid,
        stat.gid,
        stat.size,
    );
}

fn fmt_depth(depth: usize) -> String {
    " ".repeat(depth * 4)
}
