use mk_lib_database;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

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

// let mut file = client.stat("/abc/test.txt").unwrap();
// let mut file = client.open_with("/abc/test.txt", SmbOpenOptions::default().read(true)).unwrap();

#[derive(Debug, Clone)]
pub struct File_Metadata {
    pub name: String,
    pub directory: bool,
}

// tree(&client, "/");
pub fn mk_file_smb_client_tree(
    client: &SmbClient,
    uri: &str,
) -> Result<Vec<File_Metadata>, Box<dyn Error>> {
    let mut file_list: Vec<File_Metadata> = vec![];
    for entity in client.list_dir(uri)?.into_iter() {
        let entity_uri = mk_file_smb_client_entity_uri(&entity, uri);
        let is_dir = entity.get_type() == SmbDirentType::Dir;
        file_list.push(File_Metadata {
            name: entity_uri,
            directory: is_dir,
        });
    }
    Ok(file_list)
}

pub fn mk_file_smb_client_tree_smbclient(
    share_to_mount: &mk_lib_database::mk_lib_database_network_share::DBShareList,
    uri: &str,
) -> Result<Vec<File_Metadata>, Box<dyn Error>> {
    // smbclient's `-c` argument is a script: commands are separated by `;`
    // and paths are quoted with `"`. A uri containing either character could
    // inject additional smbclient commands, so reject it up front.
    if uri.contains([';', '"', '\n', '\r', '\\']) {
        return Err(format!("smbclient path contains disallowed characters: {uri:?}").into());
    }
    let mut smb_command = Command::new("smbclient");
    let share_uri = format!(
        "//{}/{}",
        share_to_mount.mm_network_share_ip, share_to_mount.mm_network_share_path
    );
    let mut smb_commands: Vec<String> =
        vec![String::from("recurse ON"), String::from("prompt OFF")];
    let cleaned_uri = uri.trim_start_matches('/');
    if cleaned_uri.is_empty() == false {
        smb_commands.push(format!("cd \"{}\"", cleaned_uri));
    }
    smb_commands.push(String::from("ls"));
    smb_command
        .arg(share_uri)
        .arg("-g")
        .arg("-c")
        .arg(smb_commands.join(";"));
    if let Some(workgroup) = share_to_mount.mm_network_share_workgroup.as_deref() {
        if workgroup.is_empty() == false {
            smb_command.arg("-W").arg(workgroup);
        }
    }
    if let Some(user) = share_to_mount.mm_share_auth_user.as_deref() {
        // The `-U user%pass` form exposes the password to anything that can
        // read this process's argv (ps, /proc). Reject `%` in the credentials
        // so a malicious password can't escape the user field; a follow-up
        // should move to an auth file via `-A` to keep the secret off argv.
        let pass = share_to_mount
            .mm_share_auth_password
            .as_deref()
            .unwrap_or_default();
        if user.contains('%') || pass.contains('%') || user.contains('\n') || pass.contains('\n') {
            return Err("smb credentials contain disallowed characters".into());
        }
        smb_command.arg("-U").arg(format!("{}%{}", user, pass));
    } else {
        smb_command.arg("-N");
    }
    let smb_output = smb_command.output()?;
    if smb_output.status.success() == false {
        return Err(format!(
            "smbclient failed with status {:?}",
            smb_output.status.code()
        )
        .into());
    }
    let stdout_data = String::from_utf8(smb_output.stdout)?;
    let mut file_list: Vec<File_Metadata> = vec![];
    for line in stdout_data.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }
        if parts[0] != "D" && parts[0] != "F" {
            continue;
        }
        if parts[1] == "." || parts[1] == ".." {
            continue;
        }
        let path_value = if parts[1].starts_with('/') {
            parts[1].to_string()
        } else if uri.ends_with('/') {
            format!("{}{}", uri, parts[1])
        } else {
            format!("{}/{}", uri, parts[1])
        };
        file_list.push(File_Metadata {
            name: path_value,
            directory: parts[0] == "D",
        });
    }
    if file_list.is_empty() {
        return Err("smbclient returned no parsable entries".into());
    }
    Ok(file_list)
}

fn mk_file_smb_client_entity_uri(entity: &SmbDirent, path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.push(PathBuf::from(entity.name()));
    p.as_path().to_string_lossy().to_string()
}
