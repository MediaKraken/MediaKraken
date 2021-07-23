use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use actix_web_grants::{GrantsMiddleware, PermissionGuard, proc_macro::has_permissions};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};
use rcgen::generate_simple_self_signed;
use serde_json::json;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_file/src/mk_lib_file.rs"]
mod mk_lib_file;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_version.rs"]
mod mk_lib_database_version;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkwebapp";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // check for and create ssl certs if needed
    if Path::new("./key/cacert.pem").exists() == false {
        mk_lib_logging::mk_logging_post_elk("info",
                                            json!({"stuff": "Cert not found, generating."}),
                                            LOGGING_INDEX_NAME).await;
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("./key/cacert.pem")?;
        file_pem.write_all(cert.serialize_pem().unwrap())?;
        let mut file_key_pem = File::create("./key/privkey.pem")?;
        file_key_pem.write_all(cert.serialize_private_key_pem())?;
    }

    // create crypto salt if needed
    if Path::new("./secure/data.zip").exists() == false {
        mk_lib_logging::mk_logging_post_elk(message_type = "info",
                                            json!({"stuff": "data.zip not found, generating."}),
                                            LOGGING_INDEX_NAME).await;
        // create the hash salt
        let mut salt;
        if Path::new("/mediakraken/secure/data.zip") == false {
            let mut file_salt = File::create("/mediakraken/secure/data.zip")?;
            const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
            let salt = [0u8; CREDENTIAL_LEN];
            file_salt.write_all(salt);
        } else {
            let salt = mk_lib_file::mk_read_file_data("/mediakraken/secure/data.zip");
        }
        /*
kdf = PBKDF2HMAC(
algorithm=hashes.SHA256(),
length=32,
salt=salt,
iterations=100000,
backend=default_backend()
)
if 'SECURE' in os.environ:  # docker compose
self.hash_key = base64.urlsafe_b64encode(
kdf.derive(os.environ['SECURE'].encode('utf-8')))
else:  # docker swarm
self.hash_key = base64.urlsafe_b64encode(
kdf.derive(common_file.com_file_load_data('/run/secrets/secure_key')))
self.fernet = Fernet(self.hash_key)

def com_hash_gen_crypt_encode(self, encode_string):
# encode, since it needs bytes
return self.fernet.encrypt(encode_string.encode())

def com_hash_gen_crypt_decode(self, decode_string):
# encode, since it needs bytes
return self.fernet.decrypt(decode_string.encode())
*/
    }

    // db version check
    let db_client = &mk_lib_database::mk_lib_database_open().await?;
    mk_lib_database_version::mk_lib_database_version_check(db_client,
                                                           true).await;

    // startup the server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}