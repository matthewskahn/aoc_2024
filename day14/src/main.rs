use std::cmp::Ordering;

use aoc_utils::read_file;
use regex::Regex;

const DIMS: (i32, i32) = (101, 103);

#[derive(Debug, Copy, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn r_move(&mut self, turns: i32) {
        self.x += self.vx * turns;
        self.y += self.vy * turns;

        while self.x < 0 {
            self.x += DIMS.0;
        }

        while self.y < 0 {
            self.y += DIMS.1;
        }

        self.x %= DIMS.0;
        self.y %= DIMS.1;
    }
}

fn main() {
    let lines = read_file("input.txt");
    part1(&lines, 100);
    part2(&lines);
}

fn part1(lines: &Vec<String>, turns: i32) {
    let parser = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let (mut ul, mut ur, mut ll, mut lr) = (0, 0, 0, 0);

    for line in lines {
        let matches = parser.captures(line).unwrap();
        let mut x = matches.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let mut y = matches.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let vx = matches.name("vx").unwrap().as_str().parse::<i32>().unwrap();
        let vy = matches.name("vy").unwrap().as_str().parse::<i32>().unwrap();

        x += vx * turns;
        y += vy * turns;

        while x < 0 {
            x += DIMS.0;
        }

        while y < 0 {
            y += DIMS.1;
        }

        x = x % DIMS.0;
        y = y % DIMS.1;

        if x < DIMS.0 / 2 {
            if y < DIMS.1 / 2 {
                ul += 1;
            } else if y > DIMS.1 / 2 {
                ll += 1;
            }
        } else if x > DIMS.0 / 2 {
            if y < DIMS.1 / 2 {
                ur += 1;
            } else if y > DIMS.1 / 2 {
                lr += 1;
            }
        }
    }

    println!("Part 1: {}", ul * ur * ll * lr);
}

fn part2(lines: &Vec<String>) {
    let parser = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for line in lines {
        let matches = parser.captures(line).unwrap();
        let x = matches.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = matches.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let vx = matches.name("vx").unwrap().as_str().parse::<i32>().unwrap();
        let vy = matches.name("vy").unwrap().as_str().parse::<i32>().unwrap();
        robots.push(Robot { x, y, vx, vy });
    }

    'outer: for i in 1..=200000 {
        let mut grid = vec![vec![" "; DIMS.0 as usize]; DIMS.1 as usize];

        for mut robot in robots.iter().copied() {
            robot.r_move(i);
            grid[robot.y as usize][robot.x as usize] = "#";
        }

        for line in grid.iter() {
            let line = line.concat();
            let mut tree_chunks = line.split_whitespace().collect::<Vec<&str>>();
            tree_chunks.sort_by(|&a, &b| {
                if a.len() < b.len() {
                    Ordering::Less
                } else if a.len() > b.len() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            if tree_chunks.len() > 0 && tree_chunks.last().unwrap().len() > 10 {
                println!("Part 2: {}", i);
                break 'outer;
            }
        }
    }
}
