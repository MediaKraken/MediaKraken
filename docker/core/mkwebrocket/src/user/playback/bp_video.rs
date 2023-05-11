use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/playback/video")]
pub async fn user_playback_video(user: User) -> Template {
    Template::render(
        "bss_user/playback/bss_user_playback_video",
        tera::Context::new().into_json(),
    )
}
