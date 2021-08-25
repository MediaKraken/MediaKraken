#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod template_base;

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

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database_sqlx/src/mk_lib_database.rs"]
mod mk_lib_database;
// #[cfg(debug_assertions)]
// #[path = "../../../../src/mk_lib_database_sqlx/src/mk_lib_database_version.rs"]
// mod mk_lib_database_version;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_file/src/mk_lib_file.rs"]
mod mk_lib_file;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
// #[cfg(not(debug_assertions))]
// #[path = "mk_lib_database_version.rs"]
// mod mk_lib_database_version;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[catch(401)]
fn general_not_authorized() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(403)]
fn general_not_administrator() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(500)]
fn general_security() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... you shouldn't be here!r?</p>
    "#)
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

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

    // db version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    // mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
    //                                                        true).await;

    rocket::build()
        .mount("/admin", routes![])
        .mount("/public", routes![])
        .mount("/user", routes![])
        .register("/", catchers![general_not_administrator, general_not_found, general_security])
        .mount("/", FileServer::from(relative!("static")))
        .manage::<sqlx::PgPool>(sqlx_pool)
        .attach(Template::custom(|engines| {
            template_base::customize(&mut engines.tera);
        }));
}
