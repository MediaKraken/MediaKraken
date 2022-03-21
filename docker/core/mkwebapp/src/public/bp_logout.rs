use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login};

#[get("/logout")]
pub fn public_logout(auth: Auth<'_>) -> Template {
    Template::render("bss_public/bss_public_logout", context! {});
    auth.logout();
}
