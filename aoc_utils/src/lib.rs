use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(path: &str) -> Vec<String> {
    let path = Path::new(&path);

    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
