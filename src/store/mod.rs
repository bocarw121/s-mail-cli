use colored::Colorize;
use redis::{Commands, Connection};
use std::collections::HashMap;

use crate::errors::internal_error;

fn get_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap_or_else(|_| {
        internal_error();
        std::process::exit(1)
    });

    let connection = match client.get_connection() {
        Ok(connection) => connection,
        Err(_) => {
            eprintln!("{}", "Could not connect to redis".red());
            std::process::exit(1)
        }
    };
    connection
}

/// stores the users email in redis cache for future use
pub fn set_data(data: String, data_type: String) {
    let mut connection = get_connection();

    let _: () = connection.set(data_type, data).unwrap_or_else(|_| {
        internal_error();
        std::process::exit(1)
    });
}

/// Retrieves and returns the url from redis cache using the alias
pub fn get_data(key: String) -> String {
    let mut connection = get_connection();
    let key: String = match connection.get(key) {
        Ok(email) => email,

        Err(_) => {
            return String::from("");
        }
    };

    key
}

/// Lists credentials
pub fn list_keys() {
    let mut connection = get_connection();
    let list: Vec<String> = match connection.keys("*") {
        Ok(list) => list,

        Err(_) => {
            eprintln!("{}", "No credentials found".red());
            return;
        }
    };

    let list_map = create_list_map(list);

    println!("{}", format!("{:#?}", list_map).green());
}

/// Checks to see if the key exists in the redis cache
fn check_values(key: &str) -> String {
    let mut connection = get_connection();

    let value: String = match connection.get(key) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("No value found");
            return String::from("");
        }
    };

    value
}

/// Creates a hashmap of the list of keys
fn create_list_map(list: Vec<String>) -> HashMap<String, String> {
    let mut list_map: HashMap<String, String> = HashMap::new();

    for key in list {
        let mut _value = String::from("");

        if key == "email" || key == "password" || key == "provider" {
            _value = check_values(&key);
        } else {
            continue;
        }

        list_map.insert(key, _value);
    }

    list_map
}
