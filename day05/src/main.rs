use aoc_utils::read_file;
use std::{cmp::Ordering, collections::HashMap, fmt::Display};

fn main() {
    let lines = read_file("./input.txt");
    part1(&lines);
    part2(&lines);
}

struct Page<'a> {
    page_num: usize,
    pages_after: &'a Vec<usize>,
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.pages_after.contains(&other.page_num) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Page<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.page_num == other.page_num
    }
}

impl Display for Page<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.page_num)
    }
}

fn part1(lines: &Vec<String>) {
    let (orderings, sequences) = split_file(lines);
    let mut sum = 0;

    let orderings = collate(orderings);

    for sequence in sequences {
        let pages: Vec<Page> = sequence
            .split(',')
            .map(|x| x.parse().unwrap())
            .map(|x| Page {
                page_num: x,
                pages_after: orderings.get(&x).unwrap(),
            })
            .collect();

        if pages.is_sorted() {
            sum += pages[pages.len() / 2].page_num;
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let (orderings, sequences) = split_file(lines);
    let mut sum = 0;

    let orderings = collate(orderings);
    for sequence in sequences {
        let mut pages: Vec<Page> = sequence
            .split(',')
            .map(|x| x.parse().unwrap())
            .map(|x| Page {
                page_num: x,
                pages_after: orderings.get(&x).unwrap(),
            })
            .collect();

        if !pages.is_sorted() {
            pages.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sum += pages[pages.len() / 2].page_num;
        }
    }

    println!("Part 2: {}", sum);
}

fn split_file(lines: &Vec<String>) -> (Vec<&String>, Vec<&String>) {
    let mut orderings: Vec<_> = Vec::new();
    let mut sequences: Vec<_> = Vec::new();

    let mut found_break = false;
    for line in lines {
        if *line == "" {
            found_break = true;
        } else if !found_break {
            orderings.push(line);
        } else {
            sequences.push(line);
        }
    }

    (orderings, sequences)
}

fn to_tuple(sequence: &str) -> (usize, usize) {
    let nums: Vec<_> = sequence.split('|').map(|x| x.parse().unwrap()).collect();
    (nums[0], nums[1])
}

fn collate(tuples: Vec<&String>) -> HashMap<usize, Vec<usize>> {
    let mut out_vec: HashMap<_, Vec<_>> = HashMap::new();

    for tuple in &tuples {
        let tuple = to_tuple(tuple);
        out_vec
            .entry(tuple.0)
            .and_modify(|e| e.push(tuple.1))
            .or_insert(vec![tuple.1]);
    }
    out_vec
}
