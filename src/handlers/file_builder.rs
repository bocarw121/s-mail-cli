use std::fs;

use colored::Colorize;
use lettre::message::{header::ContentType, Attachment, SinglePart};

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

/// Strips any paths and returns the actual filename
fn parse_file_name(filename: String) -> String {
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
            parse_file_name("/user/home/name.app".to_string()),
            "name.app".to_string()
        )
    }
}
