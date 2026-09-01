use std::fs;

pub fn load_shader(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read shader")
}
