pub mod file_builder;

use colored::Colorize;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

use crate::{
    errors::{internal_error, mailer_send_error},
    security::decrypt_password,
    store::get_data,
};

use self::file_builder::file_builder;

/// Sends the email
pub async fn send_email(
    from_email: Option<String>,
    email_to: String,
    attachment: Option<String>,
    subject: Option<String>,
) {
    let from_email = handle_email(from_email);
    let attachment = handle_option(attachment);
    let subject = handle_option(subject);

    let to_email_mbox = handle_mailbox(email_to.clone(), "Recipient email".to_string());
    let from_email_mbox = handle_mailbox(from_email.clone(), "Sender email".to_string());

    let email = match Message::builder()
        .from(from_email_mbox.clone())
        .reply_to(from_email_mbox)
        .to(to_email_mbox)
        .subject(subject)
        .singlepart(file_builder(attachment))
    {
        Ok(success) => success,
        Err(_) => {
            eprintln!("{}", format!("Error sending email to {}", email_to).red());
            return;
        }
    };

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

/// Utility function that handles the option type
fn handle_option(option: Option<String>) -> String {
    match option {
        Some(str) => str,
        None => String::from(""),
    }
}

/// If the from_email is given it will return that or it will get the default data from
/// the redis cache
fn handle_email(from_email: Option<String>) -> String {
    let from_email = handle_option(from_email);
    let stored_email = get_data("email".to_string());
    // If user passes in their email use that else get_email from redis
    if !from_email.is_empty() {
        from_email
    } else if !stored_email.is_empty() {
        stored_email
    } else {
        String::from("")
    }
}

/// Handles the the parsing and error handling of the mailbox type
pub fn handle_mailbox(mailbox_item: String, item_type: String) -> Mailbox {
    let mailbox: Mailbox = match mailbox_item.parse() {
        Ok(item) => item,
        Err(_) => {
            let item = format!("{} is Not a valid", item_type).red();
            eprintln!("{}", item);
            std::process::exit(1)
        }
    };

    mailbox
}
