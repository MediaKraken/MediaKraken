#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/internet")]
pub async fn user_inter_home(user: User) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet",
        tera::Context::new().into_json(),
    )
}
