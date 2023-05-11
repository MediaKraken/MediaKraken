use rocket::form::{Context, Contextual, Form, FromForm, FromFormField};
use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/forgot_password")]
pub async fn public_forgot_password() -> Template {
    Template::render(
        "bss_public/bss_public_forgot_password",
        tera::Context::new().into_json(),
    )
}
