from collections import deque

from common import standard_inputs, move, in_bounds, lookup, show_grid, DOWN, LEFT, UP, RIGHT


inputs = standard_inputs(18, True, True)
test = [
    'R 6 (#70c710)',
    'D 5 (#0dc571)',
    'L 2 (#5713f0)',
    'D 2 (#d2c081)',
    'R 2 (#59c680)',
    'D 2 (#411b91)',
    'L 5 (#8ceee2)',
    'U 2 (#caa173)',
    'L 1 (#1b58a2)',
    'U 2 (#caa171)',
    'R 2 (#7807d2)',
    'U 3 (#a77fa3)',
    'L 2 (#015232)',
    'U 2 (#7a21e3)'
]

DELTA_MAP = {
    'D': DOWN,
    'L': LEFT,
    'U': UP,
    'R': RIGHT
}

# 0 means R, 1 means D, 2 means L, and 3 means U
HASH_DIRS = {
    '0': 'R',
    '1': 'D',
    '2': 'L',
    '3': 'U'
}


def process_inputs(inputs):
    try:
        return [(d, int(i), hashval) for (d, i, hashval) in [line.replace('(', '').replace(')', '').replace('#', '').split(' ') for line in inputs]]
    except:
        breakpoint()
        return inputs


def part_one(inputs, display_grid=False):
    rows, cols = 1, 1
    inputs = process_inputs(inputs)
    for d, i, hashval in inputs:
        if d == 'D':
            rows += i
        elif d == 'R':
            cols += i

    rows *= 2
    cols *= 2
    position = (rows//3, cols//3)
    grid = []
    for row in range(rows):
        grid.append([0] * cols)


    path = set()
    for d, i, hashval in inputs:
        print(d, i)
        delta = DELTA_MAP[d]
        for _ in range(i):
            path.add(position)
            r, c = position
            grid[r][c] = 1
            position = move(position, delta)
            if not in_bounds(position, grid):
                print(position)
                raise ValueError
            if display_grid:
                print(position)
    if display_grid:
        print(show_grid(grid))
        breakpoint()

    starts = []
    for row in range(rows):
        walls = 0
        for col in range(cols):
            if lookup((row, col), grid) == 1:
                walls += 1
                continue
            elif walls == 1 and (row, col) not in path:
                starts.append((row, col))

    queue = deque(starts)
    while queue:
        coord = queue.pop()
        if coord in path:
            grid[r][c] = 1
        for delta in [LEFT, RIGHT, UP, DOWN]:
            next_coord = move(coord, delta)
            if in_bounds(next_coord, grid) and next_coord not in path:
                queue.appendleft(next_coord)
                path.add(next_coord)
                r, c = next_coord
                grid[r][c] = 1
                if display_grid:
                    print()
                    print(show_grid(grid))
    
    if display_grid:
        breakpoint()

    return sum([sum(row) for row in grid])


# needs work, see https://www.quora.com/Is-there-any-mathematical-algorithm-to-find-the-area-of-any-shape-using-boundary-coordinates
def part_two(inputs):
    inputs = process_inputs(inputs)
    rows, cols = 1, 1
    inputs = process_inputs(inputs)

    position = (0, 0)
    max_rows, max_cols = 0, 0
    min_rows, min_cols = float('inf'), float('inf')
    path = set()
    for d, i, hashval in inputs:
        # 0 means R, 1 means D, 2 means L, and 3 means U
        d = HASH_DIRS[hashval[-1]]
        i = int(hashval[:-1], 16)
        print(d, i)
        delta = DELTA_MAP[d]
        for _ in range(i):
            path.add(position)
            r, c = position
            min_rows = min(min_rows, r)
            min_cols = min(min_cols, c)
            max_rows = max(max_rows, r)
            max_cols = max(max_cols, c)
            position = move(position, delta)

    # odds = {i for i in range(20) if i % 2 != 0}
    total = len(path)
    for row in range(min_rows, max_rows):
        walls = 0
        for col in range(min_cols, max_cols):
            if (row, col) in path:
                walls += 1
                continue
            elif walls % 2 != 0 and (row, col) not in path:
                total += 1

    return total


if __name__ == '__main__':
    assert (result := part_one(test)) == 62, result
    print(part_one(inputs, False))
    assert (result := part_two(test)) == 952408144115, result
    print(part_two(inputs))
