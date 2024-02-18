// https://github.com/veeso/suppaftp

use std::io::Cursor;
use std::str;
use suppaftp::{FtpResult, FtpStream, ImplFtpStream};

// pub async fn mk_lib_network_ftp_connect(
//     host_ip: &str,
//     host_port: &str,
//     user_name: &str,
//     user_password: &str,
// ) -> FtpResult<ImplFtpStream<NoTlsStream>> {
//     // Create a connection to an FTP server and authenticate to it.
//     let mut ftp_stream = FtpStream::connect(format!("{}:{}", host_ip, host_port)).unwrap();
//     //let mut ftp_stream = ftp_stream.into_secure(NativeTlsConnector::from(TlsConnector::new().unwrap()), host_ip).unwrap();
//     let _ = ftp_stream.login(user_name, user_password).unwrap();
//     Ok(ftp_stream)
// }

pub async fn mk_lib_network_ftp_get_pwd(
    mut ftp_stream: suppaftp::FtpStream,
) -> Result<String, Box<dyn std::error::Error>> {
    // Get the current directory that the client will be reading from and writing to.
    let ftp_directory = ftp_stream.pwd().unwrap();
    Ok(ftp_directory)
}

pub async fn mk_lib_network_ftp_set_cwd(mut ftp_stream: suppaftp::FtpStream, new_directory: &str) {
    // Change into a new directory, relative to the one we are currently in.
    let _ = ftp_stream.cwd(new_directory).unwrap();
}

pub async fn mk_lib_network_ftp_get(
    mut ftp_stream: suppaftp::FtpStream,
    get_file_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Retrieve (GET) a file from the FTP server in the current working directory.
    let remote_file = ftp_stream.retr_as_buffer(get_file_name).unwrap();
    let ftp_data = str::from_utf8(&remote_file.into_inner())
        .unwrap()
        .to_owned();
    Ok(ftp_data)
}

pub async fn mk_lib_network_ftp_put(mut ftp_stream: suppaftp::FtpStream, put_file_name: &str) {
    // Store (PUT) a file from the client to the current working directory of the server.
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    let _ = ftp_stream.put_file(put_file_name, &mut reader);
}

pub async fn mk_lib_network_ftp_close(mut ftp_stream: suppaftp::FtpStream) {
    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
}
