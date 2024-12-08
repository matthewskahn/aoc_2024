use aoc_utils::read_file_as_chars;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

fn main() {
    let lines = read_file_as_chars("./input.txt");

    let mut antennae: HashMap<char, Vec<Node>> = HashMap::new();
    for y in 0..lines.len() {
        for x in 0..lines.len() {
            let val = lines[y][x];
            if val == '.' {
                continue;
            }
            antennae
                .entry(val)
                .and_modify(|v| {
                    v.push(Node {
                        x: x as isize,
                        y: y as isize,
                    })
                })
                .or_insert(vec![Node {
                    x: x as isize,
                    y: y as isize,
                }]);
        }
    }

    part1(&antennae, &lines);
    part2(&antennae, &lines);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    x: isize,
    y: isize,
}

impl Sub for Node {
    type Output = Node;

    fn sub(self, rhs: Self) -> Self::Output {
        Node {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Node {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Node {
    type Output = Node;

    fn mul(self, rhs: Self) -> Self::Output {
        Node {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

fn is_in_bounds(lines: &Vec<Vec<char>>, node: Node) -> bool {
    node.x < lines.len() as isize && node.x >= 0 && node.y < lines.len() as isize && node.y >= 0
}

fn part1(antennae: &HashMap<char, Vec<Node>>, lines: &Vec<Vec<char>> ) {
    let mut antinodes: HashSet<Node> = HashSet::new();

    antennae.iter().for_each(|(_, v)| {
        for node_i in 0..v.len() - 1 {
            for node_j in node_i + 1..v.len() {
                let diff = v[node_i] - v[node_j];

                if diff.x < 0 && diff.y > 0 || diff.x > 0 && diff.y < 0 {
                    let candidate = v[node_j] - diff;
                    if is_in_bounds(lines, candidate) {
                        antinodes.insert(candidate);
                    }
                    let candidate = v[node_i] + diff;
                    if is_in_bounds(lines, candidate) {
                        antinodes.insert(candidate);
                    }
                } else {
                    let candidate = v[node_i] + diff;
                    if is_in_bounds(lines, candidate) {
                        antinodes.insert(candidate);
                    }
                    let candidate = v[node_j] - diff;
                    if is_in_bounds(lines, candidate) {
                        antinodes.insert(candidate);
                    }
                }
            }
        }
    });

    println!("Part 1: {}", antinodes.len());
}

fn part2(antennae: &HashMap<char, Vec<Node>>, lines: &Vec<Vec<char>>) {
    let mut antinodes: HashSet<Node> = HashSet::new();

    antennae.iter().for_each(|(_, v)| {
        for node_i in 0..v.len() - 1 {
            for node_j in node_i + 1..v.len() {
                let diff = v[node_i] - v[node_j];

                if diff.x < 0 && diff.y > 0 || diff.x > 0 && diff.y < 0 {
                    let mut i = 0;
                    loop {
                        let candidate = v[node_j] - Node { x: i, y: i } * diff;
                        if is_in_bounds(lines, candidate) {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                        i += 1;
                    }

                    i = 0;
                    loop {
                        let candidate = v[node_i] + Node { x: i, y: i } * diff;
                        if is_in_bounds(lines, candidate) {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                        i += 1;
                    }
                } else {
                    let mut i = 0;
                    loop {
                        let candidate = v[node_i] + Node { x: i, y: i } * diff;
                        if is_in_bounds(lines, candidate) {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                        i += 1;
                    }

                    i = 0;
                    loop {
                        let candidate = v[node_j] - Node { x: i, y: i } * diff;
                        if is_in_bounds(lines, candidate) {
                            antinodes.insert(candidate);
                        } else {
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
    });

    println!("Part 2: {}", antinodes.len());
}
