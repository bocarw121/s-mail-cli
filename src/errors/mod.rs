use colored::Colorize;

/// Handles errors when sending email
pub fn mailer_send_error(str_error: String) {
    if str_error.contains("Too many bad auth attempts") {
        eprintln!("{}", "Too many bad auth attempts".red());
        std::process::exit(1)
    } else if str_error.contains("Username and Password not accepted.") {
        eprintln!("{}", "Username and password not accepted".red());
        std::process::exit(1)
    } else if str_error.contains("failed to lookup address information:") {
        eprintln!("{}", "Failed to lookup address information: double check your provider information using the credentials command".red());
        std::process::exit(1)
    } else {
        eprintln!("{}", "Error sending email".red());
        std::process::exit(1)
    }
}

pub fn internal_error() {
    eprintln!("{}", "Something went wrong, please try again later".red());
}
