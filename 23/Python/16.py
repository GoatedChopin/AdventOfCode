import copy
from collections import deque

from common import standard_inputs, lookup, in_bounds, move, show_grid, LEFT, RIGHT, DOWN, UP, DISPLAY_CHARS


inputs = standard_inputs(16, True, True)
test = [
    r'.|...\....',
    r'|.-.\.....',
    r'.....|-...',
    r'........|.',
    r'..........',
    '.........\\',
    '..../.\\\\..',
    r'.-.-/..|..',
    '.|....-|.\\',
    r'..//.|....'
]

REFLECTIONS = {
    '\\': {RIGHT: [DOWN],
           UP: [LEFT],
           LEFT: [UP],
           DOWN: [RIGHT]},
    '/': {LEFT: [DOWN],
          UP: [RIGHT],
          RIGHT: [UP],
          DOWN: [LEFT]},
    '|': {RIGHT: [UP, DOWN],
          LEFT: [UP, DOWN]},
    '-': {DOWN: [LEFT, RIGHT],
          UP: [LEFT, RIGHT]}
}


def parse_inputs(inputs):
    grid = [[c for c in line] for line in inputs]
    first_len = len(grid[0])
    for row in grid:
        assert len(row) == first_len, f"Row is not the same len: {row}, {len(row)}, {first_len}"
    return grid 


def part_one(inputs, display_grid=True, start=(0, 0), start_delta=RIGHT):
    grid = parse_inputs(inputs)
    if display_grid:
        display = copy.deepcopy(grid)

    visited = set()
    origin_deltas = REFLECTIONS.get(lookup(start, grid), {}).get(start_delta, [start_delta])
    queue = deque()
    for delta in origin_deltas:
        queue.appendleft((start, delta))

    while queue:
        coord, delta = queue.pop()
        if (coord, delta) in visited:
            # print(f"Already visited {coord}")
            continue
        # print(f"Visiting {coord}")
        visited.add((coord, delta))
        if display_grid:
            r, c = coord
            display[r][c] = DISPLAY_CHARS[delta]
            print(show_grid(display))

        next_coord = move(coord, delta)
        if in_bounds(next_coord, grid):
            next_char = lookup(next_coord, grid)
            if next_char in REFLECTIONS and delta in REFLECTIONS[next_char]:
                for next_delta in REFLECTIONS[next_char][delta]:
                    # if move(next_coord, next_delta) not in visited:
                    # print((next_coord, next_delta), 'In mirrors')
                    queue.appendleft((next_coord, next_delta))
            else:
                # print((next_coord, delta), 'Not in mirrors')
                queue.appendleft((next_coord, delta))
    
    unique_coords = {coord for coord, delta in visited}
    print(f"Starting coordinate {start} with initial delta {start_delta} covered {len(unique_coords)} tiles")
    return len(unique_coords)


def part_two(inputs, display_grid=True):
    best_position, most_tiles = (0, 0), 0
    top_walls = [((0, c), DOWN) for c in range(len(inputs[0]))]
    left_walls = [((r, 0), RIGHT) for r in range(len(inputs))]
    right_walls = [((r, len(inputs[0])-1), LEFT) for r in range(len(inputs))]
    bottom_walls = [((len(inputs)-1, c), UP) for c in range(len(inputs[0]))]
    for start, delta in top_walls + left_walls + right_walls + bottom_walls:
        if (current_tiles := part_one(inputs, display_grid, start, delta)) > most_tiles:
            best_position, most_tiles = start, current_tiles
            print(f"New best: {most_tiles} starting at {best_position}")
            # breakpoint()
    return most_tiles


if __name__ == '__main__':
    assert part_one(test) == 46
    print(part_one(inputs, True))
    assert part_two(test) == 51
    print(part_two(inputs, False))
