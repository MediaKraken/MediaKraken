use rocket::fs::{FileServer, relative};
use rocket::{Rocket, Request, Build};
use rocket::response::content::RawHtml;
use rocket::response::{content, status};
use rocket::http::Status;
use rocket_dyn_templates::{Template, tera::Tera, context};

#[catch(401)]
pub async fn general_not_authorized() -> Template {
    Template::render("bss_error/bss_error_401", context! {})
}

#[catch(403)]
pub async fn general_not_administrator() -> Template {
    Template::render("bss_error/bss_error_403", context! {})
}

#[catch(404)]
pub async fn general_not_found() -> Template {
    Template::render("bss_error/bss_error_404", context! {})
}

#[catch(500)]
pub async fn general_security() -> Template {
    Template::render("bss_error/bss_error_500", context! {})
}

#[catch(default)]
pub async fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}