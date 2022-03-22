use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/internet/flickr")]
pub fn user_inter_flickr(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet_flickr", context! {})
}

#[get("/internet/flickr_detail/<guid>")]
pub fn user_inter_flickr_detail(user: User) -> Template {
    Template::render("bss_user/internet/bss_user_internet_flickr_detail", context! {})
}
