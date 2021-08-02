#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::{Rocket, Request, Build};
use rocket::response::{content, status};
use rocket::http::Status;
use rocket_dyn_templates::{Template, tera::Tera, context};

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian,
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// http://127.0.0.1:8000/?emoji
// http://127.0.0.1:8000/?name=Rocketeer
// http://127.0.0.1:8000/?lang=ру
// http://127.0.0.1:8000/?lang=ру&emoji
// http://127.0.0.1:8000/?emoji&lang=en
// http://127.0.0.1:8000/?name=Rocketeer&lang=en
// http://127.0.0.1:8000/?emoji&name=Rocketeer
// http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
// http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[catch(404)]
fn general_not_found() -> content::Html<&'static str> {
    content::Html(r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(500)]
fn general_security() -> content::Html<&'static str> {
    content::Html(r#"
        <p>Hmm... you shouldn't be here!r?</p>
    "#)
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("public/about.html", context! {
        title: "MediaKraken About",
    })
}

#[launch]
fn rocket() -> _ {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkwebapp";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    /*
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
     */
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
        .register("/", catchers![general_not_found, general_security])
        .mount("/", FileServer::from(relative!("static")))
        // .attach(sqlx::stage())
        // .attach(Template::custom(|engines| {
        //     tera::customize(&mut engines.tera);
        // }))
}
