#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::form::{Context, Contextual, Form, FromForm, FromFormField};
use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/login")]
pub async fn public_login() -> Template {
    Template::render(
        "bss_public/bss_public_login",
        tera::Context::new().into_json(),
    )
}

#[post("/login", data = "<form>")]
pub async fn public_login_post(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    result?;
    Ok(Redirect::to("/user/home"))
}
