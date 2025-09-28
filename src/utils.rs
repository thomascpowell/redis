use std::fs;

pub fn get_full_path(path: &str) -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + path
}

pub fn exists(full_path: &String) -> bool {
    fs::exists(&full_path).ok().unwrap()
}
