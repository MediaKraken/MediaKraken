use mk_lib_database;
use mk_lib_file;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions, SmbStat};
use std::path::PathBuf;

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
    }
}
