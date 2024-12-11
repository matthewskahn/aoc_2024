use aoc_utils::read_file_as_line_of_i;

fn main() {
    let lines = &read_file_as_line_of_i("./input.txt");

    part1(lines);
    part2(lines);
}

fn part1(layout: &Vec<isize>) {
    let mut raw_layout: Vec<isize> = Vec::new();

    let mut file_id = 0usize;
    for (i, spot) in layout.iter().enumerate() {
        if i % 2 == 0 {
            (0..*spot).for_each(|_| {
                raw_layout.push(file_id as isize);
            });
            file_id += 1;
        } else {
            (0..*spot).for_each(|_| {
                raw_layout.push(-1);
            });
        }
    }

    let mut i = 0;

    loop {
        if i >= raw_layout.len() {
            break;
        }

        if raw_layout[i] == -1 {
            while raw_layout[raw_layout.len() - 1] == -1 {
                raw_layout.pop();
            }
            raw_layout[i] = raw_layout.pop().unwrap();
        }
        i += 1;
    }

    let checksum: usize = raw_layout
        .iter()
        .enumerate()
        .map(|(i, &spot)| i * spot as usize)
        .sum();

    println!("Part 1: {}", checksum);
}

#[derive(Debug)]
struct File {
    id: isize,
    size: isize,
    start_offset: isize,
}

impl File {
    fn checksum(&self) -> isize {
        (self.start_offset..self.start_offset+self.size).fold(0, |acc, x| acc + x*self.id)
    }
}

fn part2(layout: &Vec<isize>) {
    let mut files: Vec<File> = Vec::new();
    let mut free_space: Vec<File> = Vec::new();

    let mut file_id = 0isize;
    let mut start_pos = 0isize;

    for (i, val) in layout.iter().enumerate() {
        if i % 2 == 0 {
            files.push(File {
                id: file_id as isize,
                size: *val,
                start_offset: start_pos,
            });
            start_pos += *val;
            file_id += 1;
        } else {
            free_space.push(File {
                id: -1,
                size: *val,
                start_offset: start_pos,
            });
            start_pos += *val;
        }
    }

    files.reverse();
    for i in 0..files.len() {
        try_move_free_space(&mut free_space, &mut files[i]);
    }
    
    println!("Part 2: {}", files.iter().map(|f| f.checksum()).sum::<isize>());

}

fn try_move_free_space(free_space: &mut Vec<File>, file: &mut File) {
    if let Some(index) = free_space
        .iter()
        .position(|f| f.size >= file.size && f.start_offset < file.start_offset)
    {
        let available = &mut free_space[index];
        file.start_offset = available.start_offset;
        available.size -= file.size;
        available.start_offset += file.size;

        if available.size == 0 {
            free_space.remove(index);
        }
    }
}
