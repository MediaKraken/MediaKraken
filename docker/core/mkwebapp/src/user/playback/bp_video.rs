use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/playback/video")]
pub fn user_playback_video() -> Template {
    Template::render("bss_user/playback/bss_user_playback_video", context! {})
}
