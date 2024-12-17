use aoc_utils::read_file_as_chars;

#[derive(Debug)]
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
    let lines = read_file_as_chars("sample.txt");

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
        println!("Move: {}", mv);
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
}

fn do_move(board: &mut Vec<Vec<char>>, pos: Position, mv: Move) -> Position {
    let mut adjacent: Vec<char> = Vec::new();

    println!("Pos: {:?}, Mv: {:?}", pos, mv);

    let mut search_pt = Position {
        x: pos.x,
        y: pos.y,
    };
    let mut new_pos = Position{ x: pos.x, y: pos.y};

    let mut found_barrel = false;
    while board[search_pt.y.checked_add_signed(mv.y).unwrap()][search_pt.x.checked_add_signed(mv.x).unwrap()] != '#' {
        if board[search_pt.y.checked_add_signed(mv.y).unwrap()][search_pt.x.checked_add_signed(mv.x).unwrap()] == 'O' {
            if found_barrel { break; } else { found_barrel = true;}
        }
        adjacent.push(board[search_pt.y][search_pt.x]);
        search_pt.x = search_pt.x.checked_add_signed(mv.x).unwrap();
        search_pt.y = search_pt.y.checked_add_signed(mv.y).unwrap();
    }

    println!("Moved {} spaces.", adjacent.len());

    if adjacent.len() > 0 {
        let barrels = adjacent.iter().filter(|c| **c == 'O').count();
        println!("Found {} barrels", barrels);

        for _ in 0..barrels {
            board[search_pt.y][search_pt.x] = 'O';
            search_pt.x = search_pt.x.checked_add_signed(-mv.x).unwrap();
            search_pt.y = search_pt.y.checked_add_signed(-mv.y).unwrap();
        }
        board[search_pt.y][search_pt.x] = '@';
        new_pos = Position {
            x: search_pt.x,
            y: search_pt.y,
        };

        for _ in 0..adjacent.len() - barrels - 1 {
            board[search_pt.y - 1][search_pt.x - 1] = '.';
            search_pt.x = search_pt.x.checked_add_signed(-mv.x).unwrap();
            search_pt.y = search_pt.y.checked_add_signed(-mv.y).unwrap();
        }
    }

    if new_pos.x != pos.x && pos.y != new_pos.y {
        board[pos.y][pos.x] = '.';
    }
    print_board(board);
    new_pos
}

fn print_board(board: &Vec<Vec<char>>) {
    for line in board {
        println!("{}", line.iter().collect::<String>());
    }
}