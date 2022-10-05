use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;

const TOKEN_KEY: &str = "yew.token";

lazy_static! {
    static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

pub fn get_token() -> Option<String> {
    let guard = TOKEN.read();
    guard.clone()
}

pub fn set_token(token: Option<String>) {
    if let Some(val) = &token {
        LocalStorage::set(TOKEN_KEY, val).unwrap();
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut guard = TOKEN.write();
    *guard = token;
}