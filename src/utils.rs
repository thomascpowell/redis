use std::fs;

pub fn get_full_path(path: &str) -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + path
}

pub fn exists(full_path: &String) -> bool {
    fs::exists(&full_path).ok().unwrap()
}

pub fn get_temp_full_path(filename: &str) -> String {
    let temp_dir = std::env::temp_dir().join("redis_temp");
    fs::create_dir_all(&temp_dir).ok();
    temp_dir.join(filename).to_string_lossy().to_string()
}
