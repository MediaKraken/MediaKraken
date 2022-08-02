#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::form::{Context, Contextual, Form, FromForm, FromFormField};
use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[get("/forgot_password")]
pub async fn public_forgot_password() -> Template {
    Template::render(
        "bss_public/bss_public_forgot_password",
        tera::Context::new().into_json(),
    )
}
