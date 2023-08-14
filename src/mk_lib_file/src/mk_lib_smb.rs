use mk_lib_database;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions, SmbStat};
use std::error::Error;
use std::path::PathBuf;

pub async fn mk_file_smb_client_connect(
    shares_to_mount: mk_lib_database::mk_lib_database_network_share::DBShareList,
) -> Result<SmbClient, Box<dyn Error>> {
    let mut smb_workgroup = "WORKGROUP";
    if shares_to_mount.mm_network_share_workgroup.is_some() {
        smb_workgroup = shares_to_mount
            .mm_network_share_workgroup
            .as_deref()
            .unwrap_or("WORKGROUP");
    }
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}", shares_to_mount.mm_network_share_ip))
            .share(format!("/{}", shares_to_mount.mm_network_share_path))
            .password(shares_to_mount.mm_share_auth_user.unwrap())
            .username(shares_to_mount.mm_share_auth_password.unwrap())
            .workgroup(smb_workgroup),
        SmbOptions::default().one_share_per_server(true),
    )
    .unwrap();
    Ok(client)
}

pub async fn mk_file_smb_client_disconnect(client: SmbClient) {
    drop(client);
}

// tree(&client, "/");
fn tree(client: &SmbClient, uri: &str) {
    for entity in client.list_dir(uri).unwrap().into_iter() {
        let entity_uri = entity_uri(&entity, uri);
        let stat = client.stat(entity_uri.as_str()).unwrap();
        print_entry(&entity, &stat);
        // if is dir, iter directory
        if entity.get_type() == SmbDirentType::Dir {
            tree(client, entity_uri.as_str());
        }
    }
}

fn entity_uri(entity: &SmbDirent, path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.push(PathBuf::from(entity.name()));
    p.as_path().to_string_lossy().to_string()
}

fn print_entry(entity: &SmbDirent, stat: &SmbStat) {
    println!(
        "{:32}\t{}\t{}\t{}\t{:?}",
        entity.name(),
        stat.uid,
        stat.gid,
        stat.size,
        stat.modified,
    );
}
