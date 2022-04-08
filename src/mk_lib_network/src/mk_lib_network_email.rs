#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/lettre/lettre/releases

use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub async fn mk_lib_network_email_send(email_from: String, email_reply_to: String,
                                        email_to: String, email_subject: String,
                                        email_body: String, user_name: String,
                                        user_password: String) {
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
