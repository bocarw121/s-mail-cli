use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub request: Mailer,
}

#[derive(Subcommand)]
pub enum Mailer {
    /// Get instructions on how to set up your credentials
    INSTRUCTIONS,
    /// Store your email, password, and provider
    CREDENTIALS {
        /// You're email address ie name@gmail.com
        #[arg(short, long)]
        email: String,
        /// You're email  password
        #[arg(short = 'c', long = "credential")]
        password: String,
        /// The email provider ie `(smtp.gmail.com)`
        #[arg(short, long)]
        provider: String,
    },
    /// Send an email
    Send {

        /// The email you would like to send to
        #[arg(short, long)]
        to_email: String,

        /// The file you would like to send as an attachment
        #[arg(short, long)]
        attachment: Option<String>,

        /// email subject
        #[arg(short, long)]
        subject: Option<String>,
    },

    /// Get a list of all the credentials
    List,
}
