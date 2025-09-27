use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("please provide a valid file path")
}
