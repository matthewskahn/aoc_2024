use aoc_utils::read_file_as_chars;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Move {
    x: isize,
    y: isize,
}

fn main() {
    let lines = read_file_as_chars("input.txt");

    let mut board: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();
    let mut start_point = Position { x: 0, y: 0 };

    let mut split = false;
    for (i, line) in lines.iter().enumerate() {
        let test_line: String = line.iter().map(|c| c.to_string()).collect();
        if test_line.trim() == "" {
            split = true;
        } else if !split {
            if (test_line).contains("@") {
                start_point = Position {
                    y: i,
                    x: test_line.find("@").unwrap(),
                };
            }
            board.push(line.to_vec());
        } else {
            moves.append(&mut line.to_vec());
        }
    }

    part1(&mut board.clone(), moves, start_point);
}

fn part1(board: &mut Vec<Vec<char>>, moves: Vec<char>, start_point: Position) {
    let mut pos = start_point;

    for mv in moves {
        match mv {
            '^' => {
                pos = do_move(board, pos, Move { x: 0, y: -1 });
            }
            'v' => {
                pos = do_move(board, pos, Move { x: 0, y: 1 });
            }
            '<' => {
                pos = do_move(board, pos, Move { x: -1, y: 0 });
            }
            '>' => {
                pos = do_move(board, pos, Move { x: 1, y: 0 });
            }
            _ => panic!("Invalid move"),
        }
    }

    let mut accum = 0;
    for y in 0..board.len() {
        for j in 0..board[y].len() {
            if board[y][j] == 'O' {
                accum += y * 100 + j;
            }
        }
    }

    println!("Part 1: {}", accum);
}

fn do_move(board: &mut Vec<Vec<char>>, pos: Position, mv: Move) -> Position {
    let mut search_pt = Position { x: pos.x, y: pos.y };
    let mut new_pos = Position { x: pos.x, y: pos.y };

    search_pt.x = search_pt.x.checked_add_signed(mv.x).unwrap();
    search_pt.y = search_pt.y.checked_add_signed(mv.y).unwrap();

    if board[search_pt.y][search_pt.x] == '.' {
        board[pos.y][pos.x] = '.';
        board[search_pt.y][search_pt.x] = '@';
        new_pos = search_pt;
    } else if board[search_pt.y][search_pt.x] == '#' {
        // Do nothing
    } else {
        let mut barrels: Vec<Position> = vec![search_pt];
        let mut hit_wall = false;

        loop {
            search_pt.x = search_pt.x.checked_add_signed(mv.x).unwrap();
            search_pt.y = search_pt.y.checked_add_signed(mv.y).unwrap();

            if board[search_pt.y][search_pt.x] == 'O' {
                barrels.push(search_pt);
            } else if board[search_pt.y][search_pt.x] == '#' {
                hit_wall = true;
                break;
            } else {
                break;
            }
        }

        // println!("{:?}", barrels);

        if !hit_wall {
            barrels.reverse();
            for barrel in barrels {
                board[barrel.y.checked_add_signed(mv.y).unwrap()]
                    [barrel.x.checked_add_signed(mv.x).unwrap()] = 'O';
            }
            board[new_pos.y.checked_add_signed(mv.y).unwrap()]
                [new_pos.x.checked_add_signed(mv.x).unwrap()] = '@';
            board[new_pos.y][new_pos.x] = '.';
            new_pos = Position {
                x: new_pos.x.checked_add_signed(mv.x).unwrap(),
                y: new_pos.y.checked_add_signed(mv.y).unwrap(),
            };
        }
    }
    new_pos
}
