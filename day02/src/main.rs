use aoc_utils::read_file_as;
fn main() {
    let lines = read_file_as::<i32>("./input.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Vec<i32>>) {
    let mut safe = 0u32;

    for line in lines {
        if is_safe(&line).0 {
            safe += 1;
        }
    }

    println!("Part 1 - safe reports: {}", safe);
}

fn part2(lines: &Vec<Vec<i32>>) {
    let mut safe = 0u32;

    for line in lines {
        if is_dampened_safe(&line) {
            safe += 1;
        }
    }

    println!("Part 2 - safe reports: {}", safe);
}

fn is_safe(report: &Vec<i32>) -> (bool, Option<usize>) {
    let mut safe = true;
    let mut is_increasing: Option<bool> = None;
    let mut offender = None;

    for i in 0..report.len() - 1 {
        let curr = report[i];
        let next = report[i + 1];

        let delta = (next - curr).abs();

        if is_increasing == None {
            is_increasing = Some(next > curr);
        }

        if is_increasing == Some(true) {
            safe = next > curr && (1..=3).contains(&delta);
        } else {
            safe = next < curr && (1..=3).contains(&delta);
        }

        if !safe {
            offender = Some(i + 1);
            break;
        }
    }

    (safe, offender)
}

fn is_dampened_safe(report: &Vec<i32>) -> bool {
    // See if it's safe
    let (mut safe, mut offender) = is_safe(report);

    if !safe {
        // Remove the offender and try again
        let mut pruned_copy = report.clone();
        pruned_copy.remove(offender.unwrap());
        (safe, _) = is_safe(&pruned_copy);

        // If still unsafe, reverse and try again (in case the 0th element was the offender)
        if !safe {
            let mut backwards = report.clone();
            backwards.reverse();
            (_, offender) = is_safe(&backwards);
            backwards.remove(offender.unwrap());
            (safe, _) = is_safe(&backwards);
        }
    }
    safe
}
