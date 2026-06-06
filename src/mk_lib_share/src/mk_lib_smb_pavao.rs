use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions};
use std::error::Error;
use std::path::PathBuf;

pub fn mk_file_smb_client_connect(
    share_to_mount: mk_lib_database::mk_lib_database_network_share::DBShareList,
) -> Result<SmbClient, Box<dyn Error>> {
    let smb_workgroup = share_to_mount
        .mm_network_share_workgroup
        .as_deref()
        .filter(|w| !w.is_empty())
        .unwrap_or("WORKGROUP");
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}", share_to_mount.mm_network_share_ip))
            .share(format!("/{}", share_to_mount.mm_network_share_path))
            .username(
                share_to_mount
                    .mm_share_auth_user
                    .as_deref()
                    .unwrap_or_default(),
            )
            .password(
                share_to_mount
                    .mm_share_auth_password
                    .as_deref()
                    .unwrap_or_default(),
            )
            .workgroup(smb_workgroup),
        SmbOptions::default().one_share_per_server(true),
    )?;
    Ok(client)
}

pub fn mk_file_smb_client_disconnect(client: SmbClient) {
    drop(client);
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub directory: bool,
}

pub fn mk_file_smb_client_tree(
    client: &SmbClient,
    uri: &str,
) -> Result<Vec<FileMetadata>, Box<dyn Error>> {
    let mut file_list: Vec<FileMetadata> = vec![];
    for entity in client.list_dir(uri)?.into_iter() {
        let entity_uri = mk_file_smb_client_entity_uri(&entity, uri);
        let is_dir = entity.get_type() == SmbDirentType::Dir;
        file_list.push(FileMetadata {
            name: entity_uri,
            directory: is_dir,
        });
    }
    Ok(file_list)
}

fn mk_file_smb_client_entity_uri(entity: &SmbDirent, path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.push(PathBuf::from(entity.name()));
    p.as_path().to_string_lossy().to_string()
}
