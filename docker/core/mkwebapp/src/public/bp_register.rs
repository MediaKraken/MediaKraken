use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[get("/register")]
pub async fn public_register() -> Template {
    Template::render("bss_public/bss_public_register", tera::Context::new().into_json())
}

#[post("/register", data = "<form>")]
pub async fn public_register_post(sqlx_pool: &rocket::State<sqlx::PgPool>, auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    if mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new()).await.unwrap() == 1 {
        mk_lib_database_user::mk_lib_database_user_set_admin(&sqlx_pool).await;
    }
    Ok(Redirect::to("/user/home"))
}