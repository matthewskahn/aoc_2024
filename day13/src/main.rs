use std::vec;

use aoc_utils::read_file;
use regex::Regex;

fn main() {
    let lines = read_file("input.txt");

    part1(&lines, 0, 1);
    part1(&lines, 10_000_000_000_000, 2);
}

fn part1(lines: &Vec<String>, offset: isize, part_num: usize) {
    let pattern = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let prize_pattern = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    let mut tokens = 0;

    for i in 0..=lines.len() / 4 {
        let button_a = &lines[i*4];
        let button_b = &lines[i*4+1];
        let prize = &lines[i*4+2];

        let mut matrix = vec![vec![0; 3]; 2];

        let captures = pattern.captures(&button_a).unwrap();
        matrix[0][0] = captures["x"].parse::<isize>().unwrap();
        matrix[1][0] = captures["y"].parse::<isize>().unwrap();

        let captures = pattern.captures(&button_b).unwrap();
        matrix[0][1] = captures["x"].parse::<isize>().unwrap();
        matrix[1][1] = captures["y"].parse::<isize>().unwrap();

        let captures = prize_pattern.captures(&prize).unwrap();
        matrix[0][2] = captures["x"].parse::<isize>().unwrap() + offset;
        matrix[1][2] = captures["y"].parse::<isize>().unwrap() + offset;
        
        let d: isize = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
        let dx: isize = matrix[0][2] * matrix[1][1] - matrix[1][2] * matrix[0][1];
        let dy: isize = matrix[0][0] * matrix[1][2] - matrix[1][0] * matrix[0][2];

        if dx % d == 0 && dy % d == 0 {
            tokens += dx / d * 3 + dy / d;
        }
    }

    println!("Part {}: {}", part_num, tokens);
}