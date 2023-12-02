use std::{env, fs};

pub fn read_input() -> String {
    let path = env::args().nth(1).expect("Path not supplied");
    fs::read_to_string(path).expect("File not found")
}
