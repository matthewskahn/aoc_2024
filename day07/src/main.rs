use aoc_utils::read_file;

fn main() {
    let lines = read_file("./input.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut accum = 0;

    for line in lines {
        let (total, operands) = {
            let pieces: Vec<&str> = line.split(": ").collect();
            let operands: Vec<usize> = pieces[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
            (pieces[0].parse::<usize>().unwrap(), operands)
        };

        if can_match_pt_1(total, operands[0], operands[1..].to_vec()) {
            accum += total;
        }
    }

    println!("Part 1: {}", accum);
}

fn part2(lines: &Vec<String>) {
    let mut accum = 0;

    for line in lines {
        let (total, operands) = {
            let pieces: Vec<&str> = line.split(": ").collect();
            let operands: Vec<usize> = pieces[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
            (pieces[0].parse::<usize>().unwrap(), operands)
        };

        if can_match_pt_2(total, operands[0], operands[1..].to_vec()) {
            accum += total;
        }
    }

    println!("Part 2: {}", accum);
}

fn concat(accum: usize, operand: usize) -> usize {
    return (accum.to_string() + &operand.to_string()).parse::<usize>().unwrap();
}

fn can_match_pt_1(total: usize, accum: usize, operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        return (accum + operands[0] == total) || (accum * operands[0] == total);
    } else {
        let test = operands[0];
        let remaining = operands[1..].to_vec();
        let adds = can_match_pt_1(total, accum + test, remaining.clone());
        let mults = can_match_pt_1(total, accum * test, remaining);
        return adds || mults;
    }
}

fn can_match_pt_2(total: usize, accum: usize, operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        return (accum + operands[0] == total) || (accum * operands[0] == total) || concat(accum, operands[0]) == total;
    } else {
        let test = operands[0];
        let remaining = operands[1..].to_vec();
        let adds = can_match_pt_2(total, accum + test, remaining.clone());
        let mults = can_match_pt_2(total, accum * test, remaining.clone());
        let concats = can_match_pt_2(total, concat(accum, test), remaining.clone());

        return adds || mults || concats;
    }
}
