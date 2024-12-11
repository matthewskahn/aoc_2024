use memoize::memoize;
use aoc_utils::read_file_as;

fn main() {
    let line = &read_file_as::<usize>("input.txt")[0];

    both_parts(&line, 25, 1);
    both_parts(&line, 75, 2);
}

fn both_parts(line: &Vec<usize>, iterations: usize, part_number: usize) {
    let mut stones = 0;

    for stone in line {
        stones += split(*stone, iterations);
    }

    println!("Part {}: {}", part_number, stones);
}

#[memoize]
fn split(stone: usize, iterations: usize) -> usize {
    
    if iterations == 0 {
        return 1;
    }

    let stone_len = stone.to_string().len();
    if stone == 0 {
        return split(1, iterations - 1);
    } else if stone_len % 2 == 0 {
        let new_stones = (&stone.to_string()[..stone_len/2], &stone.to_string()[stone_len/2..]);
        return split(new_stones.0.parse::<usize>().unwrap(), iterations - 1) + split(new_stones.1.parse::<usize>().unwrap(), iterations - 1);
    } else {
        return split(stone * 2024, iterations - 1);
    }
}