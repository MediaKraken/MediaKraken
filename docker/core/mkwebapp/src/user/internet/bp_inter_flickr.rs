use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/internet/flickr")]
pub async fn user_inter_flickr(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet_flickr", tera::Context::new().into_json())
}

#[get("/internet/flickr_detail/<guid>")]
pub async fn user_inter_flickr_detail(user: User, guid: &str) -> Template {
    Template::render("bss_user/internet/bss_user_internet_flickr_detail", tera::Context::new().into_json())
}
