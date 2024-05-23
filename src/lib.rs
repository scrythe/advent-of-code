use std::fs;

pub fn read_input_string(filename: &str) -> String {
    let filepath = format!("./data/{filename}");
    fs::read_to_string(filepath).expect("input file doesn't exist")
}
