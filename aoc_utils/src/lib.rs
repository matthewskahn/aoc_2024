use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

pub fn read_file(path: &str) -> Vec<String> {
    let path = Path::new(&path);

    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_file_as_chars(path: &str) -> Vec<Vec<char>> {
    read_file(path)
        .iter()
        .map(|x| x.chars().collect())
        .collect()
}

pub fn read_file_as<T: FromStr>(path: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let lines_as_strings = read_file(path);
    let mut lines_as_t: Vec<Vec<T>> = Vec::new();

    for line in lines_as_strings {
        let line_as_numbers: Vec<T> = line
            .split_whitespace()
            .map(|x| x.parse::<T>().unwrap())
            .collect();
        lines_as_t.push(line_as_numbers);
    }
    lines_as_t
}
