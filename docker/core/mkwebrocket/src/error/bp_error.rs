use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[catch(401)]
pub async fn general_not_authorized() -> Template {
    Template::render("bss_error/bss_error_401", tera::Context::new().into_json())
}

#[catch(403)]
pub async fn general_not_administrator() -> Template {
    Template::render("bss_error/bss_error_403", tera::Context::new().into_json())
}

#[catch(404)]
pub async fn general_not_found() -> Template {
    Template::render("bss_error/bss_error_404", tera::Context::new().into_json())
}

#[catch(500)]
pub async fn general_security() -> Template {
    Template::render("bss_error/bss_error_500", tera::Context::new().into_json())
}

#[catch(default)]
pub async fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}
