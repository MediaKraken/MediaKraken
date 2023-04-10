#![cfg_attr(debug_assertions, allow(dead_code))]

// ftp = { version = "<version>", features = ["secure"] }
// https://github.com/mattnenterprise/rust-ftp

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use ftp::FtpStream;
use serde_json::json;
use std::io::Cursor;
use std::str;
use stdext::function_name;

pub fn mk_lib_network_ftp_connect(
    host_ip: &str,
    host_port: &str,
    user_name: &str,
    user_password: &str,
) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect(format! {"{}:{}}"}, host_ip, host_port).unwrap();
    let _ = ftp_stream.login(user_name, user_password).unwrap();
    ftp_stream
}

pub fn mk_lib_network_ftp_get_pwd(ftp_stream: ftp::FtpStream) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Get the current directory that the client will be reading from and writing to.
    let ftp_directory = ftp_stream.pwd().unwrap();
    ftp_directory
}

pub fn mk_lib_network_ftp_set_cwd(ftp_stream: ftp::FtpStream, new_directory: &str) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Change into a new directory, relative to the one we are currently in.
    let _ = ftp_stream.cwd(new_directory).unwrap();
}

pub fn mk_lib_network_ftp_get(ftp_stream: ftp::FtpStream, get_file_name: &str) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Retrieve (GET) a file from the FTP server in the current working directory.
    let remote_file = ftp_stream.simple_retr(get_file_name).unwrap();
    ftp_data = str::from_utf8(&remote_file.into_inner()).unwrap();
    ftp_data
}

pub fn mk_lib_network_ftp_put(ftp_stream: ftp::FtpStream, put_file_name: &str) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Store (PUT) a file from the client to the current working directory of the server.
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    let _ = ftp_stream.put(put_file_name, &mut reader);
}

pub fn mk_lib_network_ftp_close(ftp_stream: ftp::FtpStream) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
}
