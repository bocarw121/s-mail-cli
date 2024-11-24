use colored::Colorize;
use lettre::message::Mailbox;
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


/// Utility function that handles the option type
pub fn handle_option(option: Option<String>) -> String {
    option.unwrap_or_else(|| String::from(""))
}


/// Handles the  parsing and error handling of the mailbox type
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

/// Strips any paths and returns the actual filename
pub fn parse_file_name(filename: String) -> String {
    let filename_iter = filename.split('/');
    let filename_vec = filename_iter.collect::<Vec<&str>>();
    let parsed_file = filename_vec[filename_vec.len() - 1];

    parsed_file.to_string()
}




#[cfg(test)]
mod tests {
    use super::parse_file_name;

    #[test]
    fn parse_file_name_test() {
        assert_eq!(
            crate::utils::file_builder::parse_file_name("/user/home/name.app".to_string()),
            "name.app".to_string()
        )
    }
}
