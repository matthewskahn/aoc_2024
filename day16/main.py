import sys
import heapq

class PriorityQueue:
    def __init__(self):
        self.heap = []
    
    def push(self, priority, item):
        heapq.heappush(self.heap, (priority, item))
    
    def pop(self):
        if self.heap:
            return heapq.heappop(self.heap)
        return None
    
    def is_empty(self):
        return len(self.heap) == 0

class Point:
    x: int
    y: int

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, value):
        return isinstance(value, Point) and self.x == value.x and self.y == value.y

    def __hash__(self):
        return hash((self.x, self.y))

    def __add__(self, value):
        return Point(self.x + value.x, self.y + value.y)
    
    def __str__(self):
        return f"({self.x}, {self.y})"
    
    def __lt__(self, other):
        return True

    def rotate(self, direction):
        if self == Direction.NORTH:
            return Direction.WEST if direction == "left" else Direction.EAST
        elif self == Direction.EAST:
            return Direction.NORTH if direction == "left" else Direction.SOUTH
        elif self == Direction.SOUTH:
            return Direction.EAST if direction == "left" else Direction.WEST
        elif self == Direction.WEST:
            return Direction.SOUTH if direction == "left" else Direction.NORTH

class Direction(Point):
    NORTH = Point(0, -1)
    EAST = Point(1, 0)
    SOUTH = Point(0, 1)
    WEST = Point(-1, 0)

class Reindeer:
    position: Point
    direction: Direction

class Maze:
    def __init__(self, lines, start, end):
        self.lines = lines
        self.end = end
        self.valid_spaces = [Point(x,y) for y, _ in enumerate(lines) for x, _ in enumerate(lines[y]) if lines[y][x] != '#']
        self.distances = {space: (sys.maxsize, None) for space in self.valid_spaces}
        self.distances[start] = (0, Direction.EAST)
        self.pq = PriorityQueue()
        self.pq.push(0, (start, Direction.EAST))
        self.visited = set()

    def dijkstra(self):
        while not self.pq.is_empty():
            nearest_distance, (nearest, nearest_direction) = self.pq.pop()

            if nearest in self.visited:
                continue

            if nearest_distance > self.distances[nearest][0]:
                continue

            if nearest == self.end:
                continue

            self.visited.add(nearest)
            if nearest + nearest_direction in self.valid_spaces and nearest_distance + 1 < self.distances[nearest + nearest_direction][0]:
                self.distances[nearest + nearest_direction] = (nearest_distance + 1, nearest_direction)
                self.pq.push(nearest_distance + 1, (nearest + nearest_direction, nearest_direction))

            left = nearest_direction.rotate("left")
            if nearest + left in self.valid_spaces and nearest_distance + 1001 < self.distances[nearest + left][0]:
                self.distances[nearest + left] = (nearest_distance + 1001, left)
                self.pq.push(nearest_distance + 1001, (nearest + left, left))
            
            right = nearest_direction.rotate("right")
            if nearest + right in self.valid_spaces and nearest_distance + 1001 < self.distances[nearest + right][0]:
                self.distances[nearest + right] = (nearest_distance + 1001, right)
                self.pq.push(nearest_distance + 1001, (nearest + right, right))
            


def find_path(lines, reindeer, cost = 0):
    forward = reindeer.position + reindeer.direction
    if lines[forward.y][forward.x] == 'E':
        return cost
    elif lines[forward.y][forward.x] == '#':
        return sys.maxsize
    else:
        forward_dist = find_path(lines, Reindeer(forward, reindeer.direction), cost + 1)

    left = reindeer.position + reindeer.direction.rotate("left")
    

def main():
    with open('day16/sample.txt', 'r') as f:
        lines = f.read().splitlines()
    
    part1(lines)

def part1(lines):
    
    for y in range(0,len(lines)):
        for x in range(0, len(lines[y])):
            if lines[y][x] == 'S':
                start = Point(x,y)
            elif lines[y][x] == 'E':
                end = Point(x, y)

    maze = Maze(lines, start, end)
    maze.dijkstra()

    print(f"Part 1: {maze.distances[end][0]}")



if __name__ == '__main__':
    main()