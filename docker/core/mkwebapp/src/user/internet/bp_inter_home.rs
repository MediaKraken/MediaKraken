use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/internet")]
pub async fn user_inter_home(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet", tera::Context::new().into_json())
}
