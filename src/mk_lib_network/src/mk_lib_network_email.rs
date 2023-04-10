#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/lettre/lettre/releases

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;
use stdext::function_name;

pub async fn mk_lib_network_email_send(
    email_from: String,
    email_reply_to: String,
    email_to: String,
    email_subject: String,
    email_body: String,
    user_name: String,
    user_password: String,
) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .reply_to(email_reply_to.parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject(email_subject)
        .body(String::from(email_body))
        .unwrap();

    let creds = Credentials::new(user_name.to_string(), user_password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
