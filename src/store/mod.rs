use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::errors::internal_error;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    email: Option<String>,
    password: Option<String>,
    provider: Option<String>,
}

fn get_config_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("{}", "Could not determine home directory".red());
        std::process::exit(1);
    });

    home_dir.join("smail.json")
}

fn load_config() -> Config {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Config::default();
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| {
            eprintln!("{}", "Invalid configuration file format".red());
            Config::default()
        }),
        Err(_) => {
            eprintln!("{}", "Could not read configuration file".red());
            Config::default()
        }
    }
}

fn save_config(config: &Config) {
    let config_path = get_config_path();

    let content = serde_json::to_string_pretty(config).unwrap_or_else(|_| {
        internal_error();
        std::process::exit(1);
    });

    if let Err(_) = fs::write(&config_path, content) {
        eprintln!("{}", "Could not save configuration file".red());
        std::process::exit(1);
    }
}

/// stores the users email in configuration file for future use
pub fn set_data(data: String, data_type: String) {
    let mut config = load_config();

    match data_type.as_str() {
        "email" => config.email = Some(data),
        "password" => config.password = Some(data),
        "provider" => config.provider = Some(data),
        _ => {
            eprintln!("{}", format!("Unknown data type: {}", data_type).red());
            return;
        }
    }

    save_config(&config);
}

/// Retrieves and returns the data from configuration file using the key
pub fn get_data(key: String) -> String {
    let config = load_config();

    match key.as_str() {
        "email" => config.email.unwrap_or_default(),
        "password" => config.password.unwrap_or_default(),
        "provider" => config.provider.unwrap_or_default(),
        _ => String::new(),
    }
}

/// Lists credentials
pub fn list_keys() {
    let config = load_config();
    let mut list_map: HashMap<String, String> = HashMap::new();

    if let Some(email) = &config.email {
        list_map.insert("email".to_string(), email.clone());
    }

    if let Some(password) = &config.password {
        list_map.insert("password".to_string(), password.clone());
    }

    if let Some(provider) = &config.provider {
        list_map.insert("provider".to_string(), provider.clone());
    }

    if list_map.is_empty() {
        eprintln!("{}", "No credentials found".red());
        return;
    }

    println!("{}", format!("{:#?}", list_map).green());
}
