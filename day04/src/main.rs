use aoc_utils::read_file;
use regex::Regex;

fn main() {
    let lines = read_file("/home/matt/dev/aoc/2024/day04/input.txt");

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let regex: Vec<Regex> = vec![Regex::new(r"XMAS").unwrap(), Regex::new(r"SAMX").unwrap()];

    let rotated_45 = find_matches(&regex, &rotate_45(lines));
    let rotated_neg_45 = find_matches(&regex, &rotate_neg_45(lines));
    let rotated_90 = find_matches(&regex, &rotate_90(lines));
    let orig = find_matches(&regex, lines);

    println!(
        "Orig: {}, +45: {}, -45: {}, 90: {}",
        orig, rotated_45, rotated_neg_45, rotated_90
    );
    println!(
        "Part 1: {}",
        rotated_45 + rotated_neg_45 + rotated_90 + orig
    );
}

fn part2(lines: &Vec<String>) {
    let mut sum = 0u32;

    for i in 1..lines.len()-1 {
        for j in 1..lines.len()-1 {
            if &lines[i][j..j+1] == "A" {
                let (u, d, l, r) = (&lines[i-1][j-1..j], &lines[i+1][j+1..j+2], &lines[i-1][j+1..j+2], &lines[i+1][j-1..j]);

                if is_cross_match(u,d,l,r) {
                    sum += 1;
                } 
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn is_cross_match(u: &str, d: &str, l: &str, r: &str) -> bool {
    return ((u == "M" && d == "S") || (u == "S" && d == "M")) && ((l == "M" && r == "S") || (l == "S" && r == "M")); 
}

fn rotate_45(lines: &Vec<String>) -> Vec<String> {
    let mut out_vec: Vec<String> = Vec::new();

    for i in 0..lines.len() {
        let mut fwd = String::new();
        let mut rev = String::new();

        for j in 0..=i {
            fwd.push_str(&lines[i - j][j..j + 1]);
            rev.push_str(&lines[lines.len() - 1 - j][lines.len() - 1 - i + j..lines.len() - i + j]);
        }

        if fwd.len() >= 4 {
            out_vec.push(fwd);
            out_vec.push(rev);
        }
    }

    if lines.len() % 2 == 0 {
        out_vec.pop();
    }

    out_vec
}

fn rotate_neg_45(lines: &Vec<String>) -> Vec<String> {
    let mut out_vec: Vec<String> = Vec::new();

    for i in 0..lines.len() {
        let mut fwd = String::new();
        let mut rev = String::new();

        for j in 0..=i {
            fwd.push_str(&lines[j][lines.len() - 1 - i + j..lines.len() - i + j]);
            rev.push_str(&lines[lines.len() - 1 - i + j][j..j + 1]);
        }
                out_vec.push(fwd);
                out_vec.push(rev);
    }

    if lines.len() % 2 == 0 {
        out_vec.pop();
    }

    out_vec
}

// Only works for squares
fn rotate_90(lines: &Vec<String>) -> Vec<String> {
    let mut out_vec: Vec<String> = Vec::new();

    for i in 0..lines.len() {
        let mut out_str = String::new();
        for j in 0..lines.len() {
            out_str.push_str(&lines[j][i..i + 1]);
        }
        out_vec.push(out_str);
    }
    out_vec
}

fn find_matches(regexes: &Vec<Regex>, lines: &Vec<String>) -> u32 {
    let mut sum = 0u32;

    for line in lines {
        for regex in regexes {
            sum += regex.find_iter(line).count() as u32;
        }
    }
    sum
}
