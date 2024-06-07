use std::env;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_mail(subject: &str, body: &str) -> Result<(), String> {
    let smtp_username_res = env::var("SMTP_USERNAME");
    let smtp_username;
    if smtp_username_res.is_ok() {
        smtp_username = smtp_username_res.unwrap();
    }else{
        smtp_username = String::from("");
    }
    let smtp_password_res = env::var("SMTP_PASSWORD");
    let smtp_password;
    if smtp_password_res.is_ok() {
        smtp_password = smtp_password_res.unwrap();
    }else{
        smtp_password = String::from("");
    }
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER is not set");
    let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT is not set");
    let smtp_from = env::var("SMTP_FROM").expect("SMTP_FROM is not set");
    let emails = env::var("EMAILS").expect("EMAILS is not set");
    let emails_vec: Vec<&str> = emails.split(',').collect();

    let mut email_builder = Message::builder()
        .from(smtp_from.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN);
    for email in emails_vec {
        email_builder = email_builder.cc(email.parse().unwrap());
    }
    let email = email_builder.body(String::from(body)).unwrap();


    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let transport_builder;
    if smtp_username.is_empty() && smtp_password.is_empty() {
        transport_builder = SmtpTransport::builder_dangerous(smtp_server.as_str());
    }else{
        transport_builder = SmtpTransport::relay(smtp_server.as_str()).unwrap();
    }
    let mailer = transport_builder
        .credentials(creds)
        .port(smtp_port.parse().unwrap())
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}