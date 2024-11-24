pub mod cli;
pub mod errors;
pub mod handlers;
pub mod security;
pub mod store;
pub mod utils;
mod constants;

use crate::constants::INSTRUCTIONS_MESSAGE;

use crate::utils::check_credential_values;
use clap::Parser;
use cli::{Cli, Mailer};
use colored::Colorize;
use security::encrypt_password;
use store::{list_keys, set_data};
use utils::check_credentials_set;
use crate::handlers::email::send_email;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.request {
        Mailer::CREDENTIALS {
            email,
            password,
            provider,
        } => {
            check_credential_values(&email, &password, &provider);
            set_data(email, "email".to_string());
            encrypt_password(password);
            set_data(provider, "provider".to_string());

            println!(
                "{} {}",
                "Credentials set successfully!".green(),
                "🎉".green()
            );
        }
        Mailer::Send {
            to_email,
            attachment,
            subject,
        } => {
            // Check the credentials before sending the email
            check_credentials_set();
            send_email(to_email, attachment, subject).await
        }
        Mailer::List => list_keys(),
        Mailer::INSTRUCTIONS => {
            let title = format!("{}", INSTRUCTIONS_MESSAGE).green();

            println!("{}", title);
        }
    }
}
