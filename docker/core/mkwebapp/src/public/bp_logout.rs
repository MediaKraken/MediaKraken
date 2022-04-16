use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/logout")]
pub async fn public_logout(auth: Auth<'_>) -> Template {
    Template::render("bss_public/bss_public_logout", {});
    auth.logout();
}
