
use colored::Colorize;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use std::fs;

use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
use crate::utils::parse_file_name;


use crate::{
    errors::{internal_error, mailer_send_error},
    security::decrypt_password,
    store::get_data,
};

use crate::utils::{handle_mailbox, handle_option};


/// Sends the email
pub async fn send_email(
    email_to: String,
    attachment: Option<String>,
    subject: Option<String>,
    body: Option<String>,
) {

    let from_email =get_data("email".to_string());
    let attachment = handle_option(attachment);
    let subject = handle_option(subject);
    let body = handle_option(body);

    let to_email_mbox = handle_mailbox(email_to.clone(), "Recipient email".to_string());
    let from_email_mbox = handle_mailbox(from_email.clone(), "Sender email".to_string());
        let mut email;


    if attachment.is_empty() {
        // No attachment, just body
        email = Message::builder()
            .from(from_email_mbox)
            .to(to_email_mbox)
            .subject(subject)
            .body(body)
            .unwrap_or_else(|_| {
                internal_error();
                std::process::exit(1)
            });
    } else if body.is_empty() {
        // No body, just attachment
        let attachment_part = file_builder(attachment);
        email = Message::builder()
            .from(from_email_mbox)
            .to(to_email_mbox)
            .subject(subject)
            .multipart(MultiPart::mixed().singlepart(attachment_part))
            .unwrap_or_else(|_| {
                internal_error();
                std::process::exit(1)
            });
    } else {
        // Both attachment and body
        let attachment_part = file_builder(attachment);
        email = Message::builder()
            .from(from_email_mbox)
            .to(to_email_mbox)
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                    .singlepart(SinglePart::plain(body))
                    .singlepart(attachment_part)
            )
            .unwrap_or_else(|_| {
                internal_error();
                std::process::exit(1)
            });
    }


    let password = get_data("password".to_string());
    let provider = get_data("provider".to_string());

    let decrypted_password = decrypt_password(password);

    let creds = Credentials::new(from_email, decrypted_password);

    let mailer = SmtpTransport::relay(&provider)
        .unwrap_or_else(|_| {
            internal_error();
            std::process::exit(1)
        })
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("{}", "Email sent successfully!".green())
        }
        Err(e) => {
            let str_error = e.to_string();
            mailer_send_error(str_error)
        }
    }
}




/// Takes in the file path and returns a SinglePart struct
pub fn file_builder(filename: String) -> SinglePart {
    let file_body = match fs::read(filename.clone()) {
        Ok(file_body) => file_body,
        Err(e) => {
            if e.to_string().contains("Is a directory") {
                eprintln!("{}", "Email attachment can not be a directory".red())
            } else {
                eprintln!(
                    "{}",
                    "There was an issue with processing the attachment".red()
                )
            }

            std::process::exit(1);
        }
    };
    let content_type = match ContentType::parse("application/any") {
        Ok(content_type) => content_type,
        Err(_) => {
            eprintln!(
                "{}",
                "There was an issue with processing the attachment".red()
            );
            std::process::exit(1);
        }
    };

    Attachment::new(parse_file_name(filename)).body(file_body, content_type)
}

