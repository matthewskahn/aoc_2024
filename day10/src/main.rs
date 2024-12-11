use std::collections::HashSet;

use aoc_utils::read_file_as_u8;

type Position = (isize, isize);

fn main() {
    let input = read_file_as_u8("input.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<u8>>) {
    let mut distinct_endpoints = 0;

    for i in 0..input.len() as isize {
        for j in 0..input[i as usize].len() as isize {
            if input[i as usize][j as usize] == 0 {
                let mut paths: HashSet<Vec<Position>> = HashSet::new();
                find_path(&input, (i, j), 0, vec![(i, j)], &mut paths);
                distinct_endpoints += paths.iter().map(|path| path.last().unwrap()).collect::<HashSet<&Position>>().len();
            }
        }
    }

    println!("Part 1: {}", distinct_endpoints);
}

fn part2(input: &Vec<Vec<u8>>) {
    let mut paths: HashSet<Vec<Position>> = HashSet::new();
    for i in 0..input.len() as isize {
        for j in 0..input[i as usize].len() as isize {
            if input[i as usize][j as usize] == 0 {
                find_path(&input, (i, j), 0, vec![(i,j)], &mut paths);
            }
        }
    }

    println!("Part 2: {}", paths.len());
}

fn find_path(input: &Vec<Vec<u8>>, start: Position, elevation: u8, path_so_far: Vec<Position>, paths: &mut HashSet<Vec<Position>>) {

    for next in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let next_move = (start.0 + next.0, start.1 + next.1);
        if next_move.0 < input.len() as isize && next_move.0 >= 0 && next_move.1 < input.len() as isize && next_move.1 >= 0 && input[next_move.0 as usize][next_move.1 as usize] == elevation + 1 {
            let mut new_path = path_so_far.clone();
            new_path.push(next_move);
            if input[next_move.0 as usize][next_move.1 as usize] == 9 {
                paths.insert(new_path);
            } else {
                find_path(input, next_move, elevation + 1, new_path, paths);
            }
        }
    }
}
