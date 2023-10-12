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
use utils::check_credentials_set;

use crate::utils::check_credential_values;

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
        Mailer::INSTRUCTIONS => {
            let title = format!("Instructions on how to get your Gmail app password
  1. Go to [Google Account](https://myaccount.google.com/) and click on the avatar on the top right corner and click on `Manage your Google Account`
  2. Find the Security section on the left side
  3. In the How you sign in to Google section, Enable 2-Step Verification if not already enabled
  4. Then got to https://myaccount.google.com/apppasswords
  5. Add your application name and click on the create button.
  6. Copy the generated password.
  7. Now your ready to add your credentials run the command smail credentials -e <your email> -c <your password> -p <your provider> ie smtp.gmail.com and you're all set!
  ").green();

            println!("{}", title);
        }
    }
}
