use std::collections::HashMap;

use aoc_utils::read_file_as_chars;

type Position = (usize, usize);

fn main() {
    let lines = read_file_as_chars("input.txt");
    part1(&lines);
}

fn part1(lines: &Vec<Vec<char>>) {
    let mut perimeters: HashMap<char, usize> = HashMap::new();
    let mut areas: HashMap<char, usize> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let mut sections = 4;

            // Check left
            if x as isize - 1 > 0 && lines[y][x - 1] == *c {
                sections -= 1;
            }

            // Check right
            if x as isize + 1 < line.len() as isize && lines[y][x + 1] == *c {
                sections -= 1;
            }

            // Look up
            if y as isize - 1 > 0 && lines[y - 1][x] == *c {
                sections -= 1;
            }

            // Look down
            if y as isize + 1 < lines.len() as isize && lines[y + 1][x] == *c {
                sections -= 1;
            }

            areas.entry(*c).and_modify(|e| *e += 1).or_insert(1);
            perimeters.entry(*c).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    println!("Part 1: {}", perimeters.iter().map(|(k,v)| v*areas.get(k).unwrap()).sum::<usize>());
}

fn find_regions(lines: &Vec<Vec<char>>) -> HashMap<char, Vec<Position>> {
    let mut regions: HashMap<char, Vec<Position>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            regions.entry(*c).and_modify(|e| e.push((x, y))).or_insert(vec!((x,y)));
        }
    }
    regions
}
