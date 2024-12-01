use std::fs::read_to_string;
use std::path::Path;
use std::vec::Vec;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_file("/home/matt/dev/aoc/2024/day01/input.txt");
    
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in lines {
        let elems: Vec<&str> = line.split_whitespace().collect();
        first_list.push(elems[0].parse().unwrap());
        second_list.push(elems[1].parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut sum = 0;

    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }
    println!("Sum of part 1 is {}", sum);
}

fn part2() {
    let lines = read_file("/home/matt/dev/aoc/2024/day01/input.txt");
    
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in lines {
        let elems: Vec<&str> = line.split_whitespace().collect();
        first_list.push(elems[0].parse().unwrap());
        second_list.push(elems[1].parse().unwrap());
    }

    let mut sum: u32 = 0;

    for num in first_list {
        let hist: u32 = second_list.iter().map(|&elem| match num == elem {true => 1, _ => 0}).sum();
        sum += num * hist;
    }

    println!("Sum of part 2 is {}", sum);
}

fn read_file(path: &str) -> Vec<String> {
    let path = Path::new(&path);

    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}