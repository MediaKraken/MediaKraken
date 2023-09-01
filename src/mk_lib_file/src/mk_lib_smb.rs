use mk_lib_database;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions, SmbStat};
use std::error::Error;
use std::path::PathBuf;

pub fn mk_file_smb_client_connect(
    share_to_mount: mk_lib_database::mk_lib_database_network_share::DBShareList,
) -> Result<SmbClient, Box<dyn Error>> {
    let mut smb_workgroup = "WORKGROUP";
    if share_to_mount.mm_network_share_workgroup.is_some() {
        smb_workgroup = share_to_mount
            .mm_network_share_workgroup
            .as_deref()
            .unwrap_or("WORKGROUP");
    }
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}", share_to_mount.mm_network_share_ip))
            .share(format!("/{}", share_to_mount.mm_network_share_path))
            .username(share_to_mount.mm_share_auth_user)
            .password(share_to_mount.mm_share_auth_password.unwrap())
            .workgroup(smb_workgroup),
        SmbOptions::default().one_share_per_server(true),
    )
    .unwrap();
    println!("IP: {:?}", share_to_mount.mm_network_share_ip);
    println!("Path: {:?}", share_to_mount.mm_network_share_path);
    //println!("User: {:?}", share_to_mount.mm_share_auth_user);
    //println!("Pass: {:?}", share_to_mount.mm_share_auth_password.unwrap());
    println!("Workgroup: {:?}", smb_workgroup);
    Ok(client)
}

pub fn mk_file_smb_client_disconnect(client: SmbClient) {
    drop(client);
}

// let mut file = client.stat("/abc/test.txt").unwrap();
// let mut file = client.open_with("/abc/test.txt", SmbOpenOptions::default().read(true)).unwrap();

#[derive(Debug, Clone)]
pub struct File_Metadata {
    pub name: String,
    pub directory: bool,
}

// tree(&client, "/");
pub fn mk_file_smb_client_tree(client: &SmbClient, uri: &str) -> Vec<File_Metadata> {
    let mut file_list: Vec<File_Metadata> = vec![];
    for entity in client.list_dir(uri).unwrap().into_iter() {
        let entity_uri = mk_file_smb_client_entity_uri(&entity, uri);
        let stat = client.stat(entity_uri.as_str()).unwrap();
        let mut new_file = File_Metadata {name: entity_uri, directory: false};
        if entity.get_type() == SmbDirentType::Dir {
            new_file.directory = true;
        }
        file_list.push(new_file);
    }
    file_list
}

fn mk_file_smb_client_entity_uri(entity: &SmbDirent, path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.push(PathBuf::from(entity.name()));
    p.as_path().to_string_lossy().to_string()
}

fn mk_file_smb_client_print_entry(entity: &SmbDirent, stat: &SmbStat) {
    println!(
        "{:32}\t{}\t{}\t{}\t{:?}",
        entity.name(),
        stat.uid,
        stat.gid,
        stat.size,
        stat.modified,
    );
}