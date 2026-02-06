from collections import deque

from common import standard_inputs


inputs = standard_inputs(10, True, True)

test = [
    '.F----7F7F7F7F-7....',
    '.|F--7||||||||FJ....',
    '.||.FJ||||||||L7....',
    'FJL7L7LJLJ||LJ.L-7..',
    'L--J.L7...LJS7F-7L7.',
    '....F-J..F7FJ|L7L7L7',
    '....L7.F7||L7|.L7L7|',
    '.....|FJLJ|FJ|F7|.LJ',
    '....FJL-7.||.||||...',
    '....L---J.LJ.LJLJ...'
]

test2 = [
    '..........',
    '.S------7.',
    '.|F----7|.',
    '.||....||.',
    '.||....||.',
    '.|L-7F-J|.',
    '.|..||..|.',
    '.L--JL--J.',
    '..........'
]


LEFT = (0, -1)
RIGHT = (0, 1)
UP = (-1, 0)
DOWN = (1, 0)
PIPES = {
    '-': {
        RIGHT: RIGHT,
        LEFT: LEFT
    },
    '|': {
        DOWN: DOWN,
        UP: UP
    },
    'J': {
        RIGHT: UP,
        DOWN: LEFT
    },
    'L': {
        DOWN: RIGHT,
        LEFT: UP
    },
    '7': {
        RIGHT: DOWN,
        UP: LEFT
    },
    'F': {
        UP: RIGHT,
        LEFT: DOWN
    }
}


def process_inputs(inputs):
    pipe_map = [[c for c in line] for line in inputs]
    start = None
    for r, line in enumerate(pipe_map):
        if start:
            break
        for c, char in enumerate(line):
            if char == 'S':
                start = (r, c)
                break
    return pipe_map, start


def move(coord, delta):
    r, c = coord
    rd, cd = delta
    return r + rd, c + cd


def in_bounds(coord, pipe_map):
    r, c = coord
    return 0 <= r < len(pipe_map) and 0 <= c < len(pipe_map[0])


def lookup(coord, pipe_map):
    r, c = coord
    if in_bounds(coord, pipe_map):
        return pipe_map[r][c]
    return '.'


def part_one(inputs, return_path=False):
    pipe_map, start = process_inputs(inputs)
    for dir in (UP, LEFT, RIGHT, DOWN):
        delta = dir
        position = move(start, delta)
        path = [position]
        while (current_char := lookup(position, pipe_map)) not in ('.', 'S'):
            delta = PIPES[current_char].get(delta)
            if not delta:
                break
            position = move(position, delta)
            path.append(position)
        if current_char == 'S':
            break

    if return_path:
        return path

    if len(path) % 2 == 0:
        return len(path) // 2
    else:
        return (len(path) // 2) + 1


def part_two(pipe_map, path):
    """
    We're supposed to count the number of spaces which are completely enclosed by the path,
    but only ones which would be considered "inside" the main loop,
    kind of like if you stretched a rubber band into a weird shape and were
    trying to figure out what space was "inside" the rubber band loop.

    Even if there were some parts completely surrounded by the outside of the rubber band,
    you wouldn't consider those as "inside" the rubber band.

    I'm as confused as you are about how to solve this.

    Currently considering a hypothesis that:
        ONLY COORDINATES SEPARATED FROM THE OUTSIDE WORLD BY AN ODD NUMBER OF PIPE WALLS
            (IN AT LEAST ONE DIRECTION)
        AND NOT REACHABLE FROM THE OUTSIDE WORLD BY ANY PATH
        ARE ACTUALLY "INSIDE" THE LOOP.
            - Not sure if this will 100% track but initial eyeball tests check out.

    Use flood fill algorithm + other checks to count the coordinates.
    """
    total = 0
    vert = 0
    path = set(path)
    enclosed = set()
    for r in range(len(pipe_map)):
        for c in range(len(pipe_map[0])):
            if (r, c) in path:
                continue
            vert = 0
            ray = (r+1, c+1)
            while in_bounds(ray, pipe_map):
                if ray in path and lookup(ray, pipe_map) not in ("L", "7"):
                    vert += 1
                ray = move(ray, (1, 1))
            if vert % 2 != 0:
                enclosed.add((r, c))

    edges = [*[(0, i) for i in range(len(pipe_map[0]))],
             *[(len(pipe_map)-1, i) for i in range(len(pipe_map[0]))],
             *[(i, 0) for i in range(len(pipe_map))],
             *[(i, len(pipe_map[0])-1) for i in range(len(pipe_map))]]
    d = deque([c for c in edges if c not in path])
    visited = set()
    while d:
        position = d.pop()
        if position in enclosed:
            enclosed.remove(position)
        for delta in [UP, DOWN, LEFT, RIGHT] + [(-1, -1), (-1, 1), (1, -1), (1, 1)]:
            new_position = move(position, delta)
            if in_bounds(new_position, pipe_map) and \
                new_position not in path and \
                    new_position not in visited:
                d.appendleft(new_position)
                visited.add(new_position)

    for r, c in enclosed:
        pipe_map[r][c] = 'I'
    for r, c in visited:
        pipe_map[r][c] = '0'

    map_str = '\n'.join([''.join(row) for row in pipe_map])
    print(map_str)

    return len(enclosed)


if __name__ == '__main__':
    print(part_one(inputs))
    pipe_map, start = process_inputs(inputs)
    path = part_one(inputs, return_path=True)
    print(part_two(pipe_map, path))
