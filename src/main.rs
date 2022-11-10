pub mod cli;
pub mod errors;
pub mod handlers;
pub mod security;
pub mod store;
pub mod utils;

use clap::Parser;
use cli::{Cli, Mailer};
use colored::Colorize;
use handlers::send_email;
use security::encrypt_password;
use store::{list_keys, set_data};
use utils::{check_credential_values, check_credentials_set};

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
        Mailer::EMAIL {
            from_email,
            to_email,
            attachment,
            subject,
        } => {
            // Check the credentials before sending the email
            check_credentials_set();
            send_email(from_email, to_email, attachment, subject).await
        }
        Mailer::List => list_keys(),
    }
}
