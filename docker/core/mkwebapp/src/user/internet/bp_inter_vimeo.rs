use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/internet/vimeo")]
pub fn user_inter_vimeo() -> Template {
    Template::render("bss_user/internet/bss_user_internet_vimeo", context! {})
}

#[get("/internet/vimeo_detail/<guid>")]
pub fn user_inter_vimeo_detail(guid: &str) -> Template {
    Template::render("bss_user/internet/bss_user_internet_vimeo_detail", context! {})
}
