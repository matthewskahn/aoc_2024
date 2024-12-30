from dataclasses import dataclass


@dataclass(frozen=True)
class Point:
    x: int
    y: int

@dataclass(frozen=True)
class Barrel:
    left: Point
    right: Point

def read_input():
    with open('/home/matt/dev/aoc/2024/day15/input.txt', 'r') as f:
        return f.read().splitlines()

def expand_board(board):
    expanded_board = []
    for line in board:
        new_line = line.replace('.', '..')
        new_line = new_line.replace('O', '[]')
        new_line = new_line.replace('#', '##')
        new_line = new_line.replace('@', '@.')
        expanded_board.append(list(new_line))
    return expanded_board
   
def print_board(board):
    top_line = '    '
    second_line = '    '
    for i in range(len(board[0])):
        top_line += str(i // 10)
        second_line += str(i % 10)
    print(top_line)
    print(second_line)
    for i, line in enumerate(board):
        print(f"{i:3} {''.join(line)}")

def find_start(board):
    for y, line in enumerate(board):
        for x, char in enumerate(line):
            if char == '@':
                return Point(x, y)

def can_move(board, barrel: Barrel, direction: Point):
    if not 0 <= barrel.left.x + direction.x < len(board[0]) or not 0 <= barrel.left.y + direction.y < len(board) or not 0 <= barrel.right.x + direction.x < len(board[0]) or not 0 <= barrel.right.y + direction.y < len(board):
        return False
    # left/right move
    if direction.x == -1:
        adjacent = board[barrel.left.y][barrel.left.x + direction.x]
        if adjacent == '.':
            return True
        elif adjacent == '#':
            return False
        else:
            return can_move(board, Barrel(Point(barrel.left.x + (2*direction.x), barrel.left.y), Point(barrel.left.x + (2*direction.x) + 1, barrel.left.y)), direction)
    elif direction.x == 1:
        adjacent = board[barrel.right.y][barrel.right.x + direction.x]
        if adjacent == '.':
            return True
        elif adjacent == '#':
            return False
        else:
            l_barrel = Point(barrel.right.x + direction.x, barrel.left.y)
            r_barrel = Point(barrel.right.x + 2*direction.x, barrel.right.y)
            return can_move(board, Barrel(l_barrel, r_barrel), direction)
    else:
        adjacent_l = board[barrel.left.y + direction.y][barrel.left.x]
        adjacent_r = board[barrel.right.y + direction.y][barrel.right.x]
        if adjacent_l == '.' and adjacent_r == '.':
            return True
        elif adjacent_l == '#' or adjacent_r == '#':
            return False
        elif adjacent_l == '[' and adjacent_r == ']':
            return can_move(board, Barrel(Point(barrel.left.x + direction.x, barrel.left.y + direction.y), Point(barrel.right.x + direction.x, barrel.right.y + direction.y)), direction)
        else:
            can_move_l, can_move_r = adjacent_l == '.', adjacent_r == '.'

            if adjacent_l == ']':
                can_move_l = can_move(board, Barrel(Point(barrel.left.x - 1, barrel.left.y + direction.y), Point(barrel.left.x, barrel.left.y + direction.y)), direction)
            if adjacent_r == '[':
                can_move_r = can_move(board, Barrel(Point(barrel.right.x, barrel.right.y + direction.y), Point(barrel.right.x + 1, barrel.right.y + direction.y)), direction)
            return can_move_l and can_move_r

def move_barrel(board, barrel: Barrel, direction: Point):
    if direction.y != 0:
        adj_l = board[barrel.left.y + direction.y][barrel.left.x]
        adj_r = board[barrel.right.y + direction.y][barrel.right.x]
        if adj_l == '.' and adj_r == '.':
            board[barrel.left.y + direction.y][barrel.left.x + direction.x] = '['
            board[barrel.right.y + direction.y][barrel.right.x + direction.x] = ']'
            board[barrel.left.y][barrel.left.x] = '.'
            board[barrel.right.y][barrel.right.x] = '.'
        elif adj_l == '[' and adj_r == ']':
            move_barrel(board, Barrel(Point(barrel.left.x + direction.x, barrel.left.y + direction.y), Point(barrel.right.x + direction.x, barrel.right.y + direction.y)), direction)
            board[barrel.left.y + direction.y][barrel.left.x + direction.x] = '['
            board[barrel.right.y + direction.y][barrel.right.x + direction.x] = ']'
            board[barrel.left.y][barrel.left.x] = '.'
            board[barrel.right.y][barrel.right.x] = '.'
        else:
            if board[barrel.left.y + direction.y][barrel.left.x + direction.x] == ']':
                move_barrel(board, Barrel(Point(barrel.left.x - 1, barrel.left.y + direction.y), Point(barrel.left.x, barrel.left.y + direction.y)), direction)
            if board[barrel.right.y + direction.y][barrel.right.x + direction.x] == '[':
                move_barrel(board, Barrel(Point(barrel.right.x, barrel.right.y + direction.y), Point(barrel.right.x + 1, barrel.right.y + direction.y)), direction)
            
            board[barrel.left.y + direction.y][barrel.left.x + direction.x] = '['
            board[barrel.right.y + direction.y][barrel.right.x + direction.x] = ']'
            board[barrel.left.y][barrel.left.x] = '.'
            board[barrel.right.y][barrel.right.x] = '.'
    else:
        if direction.x == -1:
            adjacent = board[barrel.left.y][barrel.left.x + direction.x]
            if adjacent == '.':
                board[barrel.left.y][barrel.left.x + direction.x] = '['
                board[barrel.right.y][barrel.right.x + direction.x] = ']'
                board[barrel.right.y][barrel.right.x] = '.'
            else:
                move_barrel(board, Barrel(Point(barrel.left.x + 2*direction.x, barrel.left.y), Point(barrel.left.x + direction.x, barrel.right.y)), direction)
                board[barrel.left.y][barrel.left.x + direction.x] = '['
                board[barrel.right.y][barrel.right.x + direction.x] = ']'
                board[barrel.right.y][barrel.right.x] = '.'
        else:
            adjacent = board[barrel.right.y][barrel.right.x + direction.x]
            if adjacent == '.':
                board[barrel.right.y][barrel.right.x+direction.x] = ']'
                board[barrel.right.y][barrel.right.x] = '['
                board[barrel.right.y][barrel.left.x] = '.'
            else:
                move_barrel(board, Barrel(Point(barrel.right.x + direction.x, barrel.right.y), Point(barrel.right.x + 2*direction.x, barrel.right.y)), direction)
                board[barrel.right.y][barrel.right.x + direction.x] = ']'
                board[barrel.right.y][barrel.right.x] = '['
                board[barrel.left.y][barrel.left.x] = '.'
            
    
def move(board, position, direction: Point):
    adjacent = board[position.y + direction.y][position.x + direction.x]
    if adjacent == '.':
        board[position.y + direction.y][position.x + direction.x] = '@'
        board[position.y][position.x] = '.'
        return (Point(position.x + direction.x, position.y + direction.y), board)
    elif adjacent == '#':
        return (position, board)
    else:
        if direction.x == -1:
            barrel = Barrel(Point(position.x + 2*direction.x, position.y), Point(position.x + direction.x, position.y))
            if can_move(board, barrel, direction):
                move_barrel(board, barrel, direction)
                board[position.y][position.x] = '.'
                board[position.y][position.x + direction.x] = '@'
                return (Point(position.x + direction.x, position.y), board)
            else:
                return (position, board)
        elif direction.x == 1:
            barrel = Barrel(Point(position.x + direction.x, position.y), Point(position.x + 2*direction.x, position.y))
            if can_move(board, barrel, direction):
                move_barrel(board, barrel, direction)
                board[position.y][position.x] = '.'
                board[position.y][position.x + direction.x] = '@'
                return (Point(position.x + direction.x, position.y), board)
            else:
                return (position, board)
        else:
            if adjacent == ']':
                barrel = Barrel(Point(position.x - 1, position.y + direction.y), Point(position.x, position.y + direction.y))
            else:
                barrel = Barrel(Point(position.x, position.y + direction.y), Point(position.x + 1, position.y + direction.y))

            if can_move(board, barrel, direction):
                move_barrel(board, barrel, direction)
                board[position.y][position.x] = '.'
                board[barrel.right.y][barrel.right.x] = '.'
                board[barrel.left.y][barrel.left.x] = '.'
                board[position.y + direction.y][position.x] = '@'
                return (Point(position.x, position.y + direction.y), board)
            else:
                return (position, board)

def main():
    input = read_input()
    
    board = []
    moves = ''

    found_split = False
    for line in input:
        if line == '':
            found_split = True
            continue

        if not found_split:
            board.append(line)
        else:
            moves += line

    board = expand_board(board)
    position = find_start(board)

    for mv in moves:
        # print(f"Move: {mv}")
        if mv == '<':
            (position, board) = move(board, position, Point(-1, 0))
        elif mv == '>':
            (position, board) = move(board, position, Point(1, 0))
        elif mv == '^':
            (position, board) = move(board, position, Point(0, -1))
        else:
            (position, board) = move(board, position, Point(0, 1))

        # print_board(board)
    
    accum = 0
    for y in range(0,len(board)):
        for x in range(0,len(board[y])):
            if board[y][x] == '[':
                accum += 100 * y + x

    print(f"Part 2: {accum}")

if __name__ == '__main__':
    main()