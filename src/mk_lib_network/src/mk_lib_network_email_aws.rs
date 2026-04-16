use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn mk_lib_network_email_aws_transport(
    aws_region: &str,
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
) -> Result<SmtpTransport, Box<dyn std::error::Error>> {
    let smtp_endpoint = format!("email-smtp.{}.amazonaws.com", aws_region);
    let creds = Credentials::new(
        aws_access_key_id.to_string(),
        aws_secret_access_key.to_string(),
    );

    let mailer = SmtpTransport::relay(&smtp_endpoint)?
        .credentials(creds)
        .port(587)
        .build();

    Ok(mailer)
}

pub async fn mk_lib_network_email_send_aws(
    email_from: String,
    email_reply_to: String,
    email_to: String,
    email_subject: String,
    email_body: String,
    aws_region: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(email_from.parse()?)
        .reply_to(email_reply_to.parse()?)
        .to(email_to.parse()?)
        .subject(email_subject)
        .body(email_body)?;

    let mailer = mk_lib_network_email_aws_transport(
        &aws_region,
        &aws_access_key_id,
        &aws_secret_access_key,
    )?;

    mailer.send(&email)?;
    Ok(())
}
