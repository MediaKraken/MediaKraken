use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};

#[get("/forgot_password")]
pub async fn public_forgot_password() -> Template {
    Template::render("bss_public/bss_public_forgot_password", tera::Context::new().into_json())
}
