// https://github.com/lettre/lettre/releases

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

/// SMTP relay host. Defaults to Gmail; override with `MK_SMTP_RELAY`.
fn smtp_relay_host() -> String {
    std::env::var("MK_SMTP_RELAY").unwrap_or_else(|_| "smtp.gmail.com".to_string())
}

pub async fn mk_lib_network_email_send(
    email_from: String,
    email_reply_to: String,
    email_to: String,
    email_subject: String,
    email_body: String,
    user_name: String,
    user_password: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let email = Message::builder()
        .from(email_from.parse().map_err(|e| format!("invalid From {email_from:?}: {e}"))?)
        .reply_to(
            email_reply_to
                .parse()
                .map_err(|e| format!("invalid Reply-To {email_reply_to:?}: {e}"))?,
        )
        .to(email_to.parse().map_err(|e| format!("invalid To {email_to:?}: {e}"))?)
        .subject(email_subject)
        .body(email_body)?;

    let creds = Credentials::new(user_name, user_password);
    let relay = smtp_relay_host();
    let mailer = SmtpTransport::relay(&relay)?.credentials(creds).build();
    mailer.send(&email)?;
    Ok(())
}
