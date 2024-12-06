use aoc_utils::read_file_as_chars;

fn main() {
    // let lines = read_file("./input.txt");
    let lines = read_file_as_chars("./input.txt");
    let path = part1(&lines, false);
    println!("Part 1: {}", path.0.len());
    part2(&lines, &path.0);
}

type Direction = (isize, isize);
const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const EAST: Direction = (0, 1);
const WEST: Direction = (0, -1);

fn part1(lines: &Vec<Vec<char>>, include_dupes: bool) -> (Vec<Direction>, bool) {
    let mut position = get_starting_point(lines);
    let mut direction = NORTH;
    let mut positions: Vec<Direction> = vec![position];
    let dimension = lines.len() as isize;

    loop {
        let next = try_move(position, direction);
        if is_out_of_bounds(dimension, next) {
            break;
        } else if is_pillar(lines, next) {
            direction = turn(direction);
        } else if is_loop(&positions) {
            return (positions, true);
        } else {
            // print(lines, next, direction);
            position = next;
            if include_dupes {
                positions.push(position);
            } else {
                if !positions.contains(&position) {
                    positions.push(position);
                }
            }
        }
    }
    (positions, false)
}

fn part2(lines: &Vec<Vec<char>>, path: &Vec<Direction>) {
    let mut options = 0u32;
    let path = &path[1..];

    for step in path {
        let mut lines = lines.clone();
        let mut line = lines.get(step.0 as usize).unwrap().clone();
        line[step.1 as usize] = '#';
        lines[step.0 as usize] = line;

        let (_, looped) = part1(&lines, true);
        if looped {
            options += 1;
        }
    }

    println!("Part 2: {options}");
}

fn get_starting_point(lines: &Vec<Vec<char>>) -> Direction {
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&'^') {
            return (
                i as isize,
                line.iter().position(|&c| c == '^').unwrap() as isize,
            );
        }
    }
    panic!("No starting point found");
}

fn is_pillar(lines: &Vec<Vec<char>>, position: Direction) -> bool {
    let line = lines.get(position.0 as usize).unwrap();
    line[position.1 as usize] == '#'
}

fn is_out_of_bounds(dimension: isize, position: (isize, isize)) -> bool {
    position.0 < 0 || position.1 < 0 || position.0 >= dimension || position.1 >= dimension
}

fn turn(direction: Direction) -> Direction {
    match direction {
        NORTH => EAST,
        EAST => SOUTH,
        SOUTH => WEST,
        WEST => NORTH,
        _ => panic!("Invalid direction"),
    }
}

fn try_move(position: Direction, direction: Direction) -> Direction {
    (position.0 + direction.0, position.1 + direction.1)
}

fn is_loop(positions: &Vec<Direction>) -> bool {
    if positions.len() >= 2 {
        let needle = positions[positions.len() - 2..].to_vec();

        for i in 0..positions.len() - 2 {
            let slice = positions[i..i + 2].to_vec();
            if slice[0] == needle[0] && slice[1] == needle[1] {
                return true;
            }
        }
    }
    false
}
