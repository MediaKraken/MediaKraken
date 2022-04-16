use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};

#[get("/register")]
pub async fn public_register() -> Template {
    Template::render("bss_public/bss_public_register", tera::Context::new().into_json())
}

#[post("/register", data = "<form>")]
pub async fn public_register_post(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to("/user/home"))
}