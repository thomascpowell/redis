pub fn get_full_path(path: &str) -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + path
}
