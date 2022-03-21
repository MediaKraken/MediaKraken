use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login};

#[get("/internet/home")]
pub fn user_internet_home(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet", context! {})
}
