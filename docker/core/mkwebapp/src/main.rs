#[macro_use]
extern crate rocket;

use rocket::{Rocket, Request, Build};
use rocket::response::{content, status};
use rocket::http::Status;
//use rocket_dyn_templates::Template;

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

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
        .register("/", catchers![general_not_found])
        //.attach(sqlx::stage())
    // .attach(Template::custom(|engines| {
    //     tera::customize(&mut engines.tera);
    // }))
}
