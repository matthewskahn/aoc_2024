use std::collections::HashMap;

use aoc_utils::read_file_as_chars;

type Position = (usize, usize);

fn main() {
    let lines = read_file_as_chars("sample.txt");
    let regions: HashMap<char, Vec<Vec<Position>>> = find_regions(&lines);
    part1(&lines, &regions);
    part2(&lines, &regions);
}

fn part1(lines: &Vec<Vec<char>>, regions: &HashMap<char, Vec<Vec<Position>>>) {
    let mut accum = 0;

    for (_, region) in regions {
        for ind in region {
            accum += ind.len() * find_perimeter(&ind, lines);
        }
    }

    println!("Part 1: {}", accum);
}

fn part2(lines: &Vec<Vec<char>>, regions: &HashMap<char, Vec<Vec<Position>>>) {
    for (code, region) in regions {
        for ind in region {
            let corners = ind.iter().map(|&e| get_corners(e, lines)).sum::<usize>();
            println!("A region of {} plants with price {} * {} = {}", code, ind.len(), corners, ind.len() * corners);
        }
    }
}

fn find_perimeter(points: &Vec<Position>, lines: &Vec<Vec<char>>) -> usize {
    let mut perimeter = 0;
    for point in points {
        let mut sections = 4;

        // Check left
        if point.0 as isize - 1 >= 0 && lines[point.1][point.0 - 1] == lines[point.1][point.0] {
            sections -= 1;
        }

        // Check right
        if point.0 as isize + 1 < lines[point.1].len() as isize
            && lines[point.1][point.0 + 1] == lines[point.1][point.0]
        {
            sections -= 1;
        }

        // Look up
        if point.1 as isize - 1 >= 0 && lines[point.1 - 1][point.0] == lines[point.1][point.0] {
            sections -= 1;
        }

        // Look down
        if point.1 as isize + 1 < lines.len() as isize
            && lines[point.1 + 1][point.0] == lines[point.1][point.0]
        {
            sections -= 1;
        }

        perimeter += sections;
    }
    perimeter
}

fn is_adjacent(point: &Position, other: &Position) -> bool {
    (point.0 as isize - 1 == other.0 as isize && point.1 == other.1)
        || (point.0 == other.0 && point.1 as isize - 1 == other.1 as isize)
        || (point.0 as isize + 1 == other.0 as isize && point.1 == other.1)
        || (point.0 == other.0 && point.1 as isize + 1 == other.1 as isize)
}

fn find_regions(lines: &Vec<Vec<char>>) -> HashMap<char, Vec<Vec<Position>>> {
    let mut regions: HashMap<char, Vec<Vec<Position>>> = HashMap::new();
    let mut charmap: HashMap<char, Vec<Position>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            charmap.entry(*c).and_modify(|v| v.push((x, y))).or_insert(vec![(x, y)]);
        }
    }

    for (code, mut points) in charmap {
        regions.insert(code, vec![]);
        while !points.is_empty() {
            let mut region = vec![points[0]];
            points.remove(0);
            
            let mut found_adjacent = true;
            while found_adjacent {
                found_adjacent = false;
                let mut i = 0;
                while i < points.len() {
                    if region.iter().any(|p| is_adjacent(p, &points[i])) {
                        region.push(points[i]);
                        points.remove(i);
                        found_adjacent = true;
                    } else {
                        i += 1;
                    }
                }
            }
            
            regions.get_mut(&code).unwrap().push(region);
        }
    }
    regions
}

fn get_corners(point: Position, lines: &Vec<Vec<char>>) -> usize {
    let mut corners = 0;

    let left = point.0 as isize - 1 < 0 || lines[point.1][point.0 - 1] != lines[point.1][point.0];
    let right = point.0 as isize + 1 >= lines.len() as isize || lines[point.1][point.0 + 1] != lines[point.1][point.0];
    let up = point.1 as isize - 1 < 0 || lines[point.1 - 1][point.0] != lines[point.1][point.0];
    let down = point.1 as isize + 1 >= lines.len() as isize || lines[point.1 + 1][point.0] != lines[point.1][point.0];
    let ul = point.1 as isize - 1 < 0 || point.0 as isize - 1 < 0 || lines[point.1 - 1][point.0 - 1] != lines[point.1][point.0];
    let ur = point.1 as isize - 1 < 0 || point.0 as isize + 1 >= lines.len() as isize || lines[point.1 - 1][point.0 + 1] != lines[point.1][point.0];
    let ll = point.1 as isize + 1 >= lines.len() as isize || point.0 as isize - 1 < 0 || lines[point.1 + 1][point.0 - 1] != lines[point.1][point.0];
    let lr = point.1 as isize + 1 >= lines.len() as isize || point.0 as isize + 1 >= lines.len() as isize || lines[point.1 + 1][point.0 + 1] != lines[point.1][point.0];
    
    if up && left {
        corners += 1;
    }
    if up && right {
        corners += 1;
    }
    if down && left {
        corners += 1;
    }
    if down && right {
        corners += 1;
    }
    if left && up && right {
        if ll { corners += 1 }
        if lr { corners += 1 }
    }
    if left && up && down {
        if ur { corners += 1 }
        if lr { corners += 1 }
    }
    if right && up && down {
        if ul { corners += 1 }
        if ll { corners += 1 }
    }
    if left && down && right {
        if ul { corners += 1 }
        if ur { corners += 1 }
    }

    if left && ll {
        corners += 1
    }

    println!("Found (up, down, left, right) -> ({},{},{},{}) for {:?} -> {} corners", up, down, left, right, point, corners);

    corners
}