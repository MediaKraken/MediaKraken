use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/internet/vimeo")]
pub async fn user_inter_vimeo(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet_vimeo", {})
}

#[get("/internet/vimeo_detail/<guid>")]
pub async fn user_inter_vimeo_detail(user: User, guid: &str) -> Template {
    Template::render("bss_user/internet/bss_user_internet_vimeo_detail", {})
}
