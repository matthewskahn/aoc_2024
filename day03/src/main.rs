use aoc_utils::read_file;
use regex::Regex;

fn main() {
    let lines = read_file("/home/matt/dev/aoc/2024/day03/input.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let pattern = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

    let mut sum = 0u32;

    for line in lines {
        sum += pattern.captures_iter(&line).map(|cap| {
            let first = cap.name("first").unwrap().as_str().parse::<u32>().unwrap();
            let second = cap.name("second").unwrap().as_str().parse::<u32>().unwrap();
            first*second
        }).sum::<u32>();
    }

    println!("Part 1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let instr_pattern = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

    let mut toggle = true;
    let mut sum = 0u32;

    for line in lines {
        pattern.find_iter(&line).for_each(|found| {
            let found_item = found.as_str();
            
            if found_item == "do()" {
                toggle = true;
            } else if found_item == "don't()" {
                toggle = false;
            } else {
                if toggle {
                    let caps = instr_pattern.captures(&found_item).unwrap();
                    let first = caps.name("first").unwrap().as_str().parse::<u32>().unwrap();
                    let second = caps.name("second").unwrap().as_str().parse::<u32>().unwrap();
                    sum += first*second;
                }
            }
        });
    }

    println!("Part 2: {}", sum);
}
