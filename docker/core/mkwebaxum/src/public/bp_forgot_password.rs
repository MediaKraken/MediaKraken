use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};

#[derive(Template)]
#[template(path = "bss_public/bss_public_forgot_password.html")]
struct ForgotPasswordTemplate;

pub async fn public_forgot_password() -> impl IntoResponse {
    let template = ForgotPasswordTemplate {};
    let reply_html = template.render().map_err(|e| e.to_string())?;
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn public_forgot_password_post() -> impl IntoResponse {
    // Password reset functionality is not yet implemented.
    // This endpoint exists to prevent the form from returning a 404.
    // A proper implementation would:
    //   1. Accept an email address
    //   2. Generate a time-limited, single-use reset token
    //   3. Send an email with a reset link containing the token
    //   4. Validate the token on the reset endpoint
    (
        StatusCode::OK,
        Html("Password reset requests are not yet enabled.".to_string()),
    )
        .into_response()
}
