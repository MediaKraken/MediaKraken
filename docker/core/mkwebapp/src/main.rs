#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use rcgen::generate_simple_self_signed;
use ring::digest;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use rocket::fs::{FileServer, relative};
use rocket::{Rocket, Request, Build};
use rocket::response::content::RawHtml;
use rocket::response::{content, status};
use rocket::http::Status;
use std::collections::{HashMap, BTreeMap};
use rocket_dyn_templates::Template;
use serde_json::json;

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "error/bp_error.rs"]
mod bp_error;

#[path = "public/bp_about.rs"]
mod bp_public_about;
#[path = "public/bp_forgot_password.rs"]
mod bp_public_forgot_password;
#[path = "public/bp_login.rs"]
mod bp_public_login;
#[path = "public/bp_register.rs"]
mod bp_public_register;

#[rocket::main]
async fn main() {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkwebapp";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // check for and create ssl certs if needed
    if Path::new("/mediakraken/key/cacert.pem").exists() == false {
        mk_lib_logging::mk_logging_post_elk("info",
                                            json!({"stuff": "Cert not found, generating."}),
                                            LOGGING_INDEX_NAME).await;
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("/mediakraken/key/cacert.pem").unwrap();
        file_pem.write_all(cert.serialize_pem().unwrap().as_bytes()).unwrap();
        let mut file_key_pem = File::create("/mediakraken/key/privkey.pem").unwrap();
        file_key_pem.write_all(cert.serialize_private_key_pem().as_bytes()).unwrap();
    }

    // create crypto salt if needed
    if Path::new("/mediakraken/secure/data.zip").exists() == false {
        mk_lib_logging::mk_logging_post_elk("info",
                                            json!({"stuff": "data.zip not found, generating."}),
                                            LOGGING_INDEX_NAME).await;
        // create the hash salt
        if Path::new("/mediakraken/secure/data.zip").exists() == false {
            let mut file_salt = File::create("/mediakraken/secure/data.zip").unwrap();
            const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
            let salt = [0u8; CREDENTIAL_LEN];
            file_salt.write_all(&salt);
        }
        let salt = mk_lib_file::mk_read_file_data("/mediakraken/secure/data.zip");
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           true).await;

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/admin", routes![])
        .mount("/public", routes![bp_public_about::public_about,
            bp_public_forgot_password::public_forgot_password,
            bp_public_login::public_login,
            bp_public_register::public_register])
        .mount("/user", routes![])
        .register("/", catchers![bp_error::general_not_authorized,
            bp_error::general_not_administrator,
            bp_error::general_not_found,
            bp_error::general_security,
            bp_error::default_catcher])
        .manage::<sqlx::PgPool>(sqlx_pool)
        .launch().await;
}
