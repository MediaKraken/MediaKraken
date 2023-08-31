use chrono::prelude::*;
use chrono::DateTime;
use mk_lib_common;
use mk_lib_database;
use mk_lib_file;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions, SmbStat};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await;
    // determine directories to audit
    for row_data in
        mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_audit_read(
            &sqlx_pool,
        )
        .await
        .unwrap()
    {
        println!("row: {:?}", row_data);
        let share_info =
            mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
                &sqlx_pool,
                row_data.mm_media_dir_share_guid,
            )
            .await
            .unwrap();
        println!("share: {:?}", share_info);
        let smb_client = mk_lib_file::mk_lib_smb::mk_file_smb_client_connect(share_info);
        match smb_client {
            Ok(smb_client) => {
                // make sure the path still exists
                let data_stat = smb_client.stat(format!("/{}", row_data.mm_media_dir_path));
                println!("{:?}", data_stat);
                match data_stat {
                    Ok(file_stat) => {
                        let last_modified =
                            mk_lib_common::mk_lib_common_date::system_time_to_date_time(
                                file_stat.modified,
                            );
                        println!(
                            "last: {:?}    diff: {:?}",
                            last_modified, row_data.mm_media_dir_last_scanned
                        );
                        if last_modified > row_data.mm_media_dir_last_scanned {
                            // grab initial file/dir list
                            let files_to_process = mk_lib_file::mk_lib_smb::mk_file_smb_client_tree(
                                &smb_client,
                                format!("/{}", row_data.mm_media_dir_path).as_str(),
                            );
                            println!("FTP: {:?}", files_to_process);
                            mediascan_file_process(
                                &smb_client,
                                files_to_process,
                                row_data.mm_media_dir_last_scanned,
                            );
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}

fn mediascan_file_process(
    smb_client: &SmbClient,
    files_to_process: Vec<mk_lib_file::mk_lib_smb::File_Metadata>,
    dir_last_scanned: DateTime<Utc>,
) {
    let epoch: DateTime<Utc> = DateTime::<Utc>::from(UNIX_EPOCH);
    for file_metadata in files_to_process.iter() {
        println!("File Process: {:?}", file_metadata);
        let mut scan_file = false;
        // TODO if stat changed.....for existing file....need to ffmpeg anyways
        if dir_last_scanned == epoch {
            scan_file = true
        }
        if scan_file == false {
            let data_stat = smb_client.stat(format!("/{}", file_metadata.name)).unwrap();
            let last_modified =
                mk_lib_common::mk_lib_common_date::system_time_to_date_time(data_stat.modified);
            if last_modified > dir_last_scanned {
                scan_file = true
            }
        }
        if scan_file {
            // TODO submit rabbitmq record to do ffmpeg scan
            // TODO submit rabbitmq record to do roku scan
            // TODO add to database if it doesn't exist
        }
    }
}
