use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::{Flash, IncomingFlashes};
use serde::Deserialize;

const MIN_USERNAME_LEN: usize = 3;
const MAX_USERNAME_LEN: usize = 64;
const MIN_PASSWORD_LEN: usize = 10;
const MAX_PASSWORD_LEN: usize = 128;

#[derive(Template)]
#[template(path = "bss_public/bss_public_register.html")]
struct RegisterTemplate {
    flash_messages: Vec<String>,
}

pub async fn public_register(flashes: IncomingFlashes) -> impl IntoResponse {
    let template = RegisterTemplate {
        flash_messages: flashes
            .iter()
            .map(|(_, message)| message.to_string())
            .collect(),
    };
    match template.render() {
        Ok(reply_html) => (flashes, Html(reply_html)).into_response(),
        Err(error) => {
            tracing::error!(?error, "register template render failed");
            Redirect::to("/error/500").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterInput {
    username: String,
    password: String,
}

fn validate_credentials(username: &str, password: &str) -> Result<(), &'static str> {
    let trimmed_user = username.trim();
    if trimmed_user.len() < MIN_USERNAME_LEN || trimmed_user.len() > MAX_USERNAME_LEN {
        return Err("Username must be between 3 and 64 characters.");
    }
    if !trimmed_user
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '@')
    {
        return Err("Username contains invalid characters.");
    }
    if password.len() < MIN_PASSWORD_LEN || password.len() > MAX_PASSWORD_LEN {
        return Err("Password must be between 10 and 128 characters.");
    }
    Ok(())
}

pub async fn public_register_post(
    State(state): State<AppState>,
    flash: Flash,
    Form(input_data): Form<RegisterInput>,
) -> (Flash, Redirect) {
    if let Err(message) = validate_credentials(&input_data.username, &input_data.password) {
        return (flash.error(message), Redirect::to("/public/register"));
    }

    // Rely on the unique constraint (or the insert itself) for the existence
    // check so we close the TOCTOU window between "does user exist" and "insert".
    let user_found = match mk_lib_database::mk_lib_database_user::mk_lib_database_user_exists(
        &state.sqlx_pool_rw,
        &input_data.username,
    )
    .await
    {
        Ok(found) => found,
        Err(error) => {
            tracing::error!(?error, "user existence check failed");
            return (
                flash.error("Unable to process registration right now."),
                Redirect::to("/public/register"),
            );
        }
    };
    if user_found {
        return (
            flash.error("User already exists!"),
            Redirect::to("/public/register"),
        );
    }

    let user_id: i64 = match mk_lib_database::mk_lib_database_user::mk_lib_database_user_insert(
        &state.sqlx_pool_rw,
        &input_data.username,
        &input_data.password,
    )
    .await
    {
        Ok(id) => id,
        Err(error) => {
            tracing::error!(?error, "user insert failed");
            return (
                flash.error("Unable to create account right now."),
                Redirect::to("/public/register"),
            );
        }
    };

    // Use 2 because id=1 is the guest user. If multiple registrations race,
    // whichever sees count==2 first wins admin; the others do not escalate.
    match mk_lib_database::mk_lib_database_user::mk_lib_database_user_count(
        &state.sqlx_pool_rw,
        String::new(),
    )
    .await
    {
        Ok(2) => {
            if let Err(error) =
                mk_lib_database::mk_lib_database_user::mk_lib_database_user_set_admin(
                    &state.sqlx_pool_rw,
                    user_id,
                )
                .await
            {
                tracing::error!(user_id, ?error, "failed to promote first user to admin");
            }
        }
        Ok(_) => {}
        Err(error) => {
            tracing::error!(?error, "user count check failed after registration");
        }
    }

    (flash, Redirect::to("/public/login"))
}
