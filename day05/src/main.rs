use aoc_utils::read_file;
use std::collections::HashMap;

fn main() {
    let lines = read_file("/home/matt/dev/aoc/2024/day05/input.txt");
    part1(&lines);
}

fn part1(lines: &Vec<String>) {
    let (orderings, sequences) = split_file(lines);
    println!("{:?}", collate(orderings));

}

fn split_file(lines: &Vec<String>) -> (Vec<&String>, Vec<&String>) {
    let mut orderings: Vec<&String> = Vec::new();
    let mut sequences: Vec<&String> = Vec::new();

    let mut found_break = false;
    for line in lines {
        if *line == "" {
            found_break = true;
        }
        else if !found_break {
            orderings.push(line);
        } else {
            sequences.push(line);
        }
    }

    (orderings, sequences)
}

fn to_tuple(sequence: &str) -> (u32,u32) {
    let nums: Vec<u32> = sequence.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
    (nums[0], nums[1])
}

fn collate(tuples: Vec<&String>) -> HashMap<u32,Vec<u32>> {
    let mut out_vec: HashMap<u32, Vec<u32>> = HashMap::new();

    for tuple in &tuples {
        let tuple = to_tuple(tuple);
        out_vec.entry(tuple.0)
            .and_modify(|e| e.push(tuple.1))
            .or_insert(vec![tuple.1]);
    }
    out_vec
}
