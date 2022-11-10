use colored::Colorize;

use crate::store::get_data;

/// Check if credential values have been set
pub fn check_credentials_set() {
    let email = get_data("email".to_string());
    let password = get_data("password".to_string());
    let provider = get_data("provider".to_string());

    if email.is_empty() || password.is_empty() || provider.is_empty() {
        eprintln!(
            "{} {}",
            "Please set your credentials first!".red(),
            "You can do this by running `smail credentials` with the email, password, and provider"
                .red()
        );

        std::process::exit(1)
    }
}

/// Make sure no empty values are passed in to any credentials
pub fn check_credential_values(email: &str, credential: &str, provider: &str) {
    if email.is_empty() || credential.is_empty() || provider.is_empty() {
        eprintln!("{}", "Credential values can not be empty".red());
        std::process::exit(1)
    }
}
