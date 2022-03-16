use rocket::fs::{FileServer, relative};
use rocket::{Rocket, Request, Build};
use rocket::response::content::RawHtml;
use rocket::response::{content, status};
use rocket::http::Status;

#[catch(401)]
pub fn general_not_authorized() -> content::RawHtml<&'static str> {
    // @common_global.jinja_template.template('bss_error/bss_error_401.html')
    content::RawHtml(r#"
        <p>401</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

// #[get("/about")]
// pub fn public_about() -> Template {
//     Template::render("templates/bbs_public_about.html", context! {
//         title: "About",
//     })
// }

#[catch(403)]
pub fn general_not_administrator() -> content::RawHtml<&'static str> {
    // @common_global.jinja_template.template('bss_error/bss_error_403.html')
    content::RawHtml(r#"
        <p>403</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(404)]
pub fn general_not_found() -> content::RawHtml<&'static str> {
    // @common_global.jinja_template.template('bss_error/bss_error_404.html')
    content::RawHtml(r#"
        <p>404</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#)
}

#[catch(500)]
pub fn general_security() -> content::RawHtml<&'static str> {
    // @common_global.jinja_template.template('bss_error/bss_error_500.html')
    content::RawHtml(r#"
        <p>500</p>
    "#)
}

#[catch(default)]
pub fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}