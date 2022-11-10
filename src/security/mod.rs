use crate::store::set_data;
use base64::{decode, encode};

pub fn encrypt_password(password: String) {
    let password_hash = encode(password);

    set_data(password_hash, "password".to_string())
}

pub fn decrypt_password(password: String) -> String {
    let hash_vec = match decode(password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };

    hash_vec.into_iter().map(|c| c as char).collect()
}
