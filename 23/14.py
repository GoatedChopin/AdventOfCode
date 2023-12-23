import numpy as np

from common import standard_inputs, in_bounds, move, lookup


inputs = standard_inputs(14, True, True)
test = [
'O....#....',
'O.OO#....#',
'.....##...',
'OO.#O....O',
'.O.....O#.',
'O.#..O.#.#',
'..O..#O..O',
'.......O..',
'#....###..',
'#OO..#....'
]


def process_inputs(inputs):
    return np.array([[c for c in line] for line in inputs])


def grid_step(grid, dir=(-1, 0)):
    moved = False
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if lookup((row, col), grid) == 'O':
                fall_coord = move((row, col), dir)
                if in_bounds(fall_coord, grid) and lookup(fall_coord, grid) == '.':
                    nr, nc = fall_coord
                    grid[row][col] = '.'
                    grid[nr][nc] = 'O'
                    moved = True
    return grid, moved


def part_one(inputs):
    grid = process_inputs(inputs)

    moved = True
    while moved:
        grid, moved = grid_step(grid)
    
    total = 0
    val = len(grid)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if lookup((row, col), grid) == 'O':
                total += val
        val -= 1
    return total


def part_two(inputs):
    grid = process_inputs(inputs)

    moved = True
    while moved:
        grid, moved = grid_step(grid)
    
    total = 0
    val = len(grid)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if lookup((row, col), grid) == 'O':
                total += val
        val -= 1
    return total


if __name__ == '__main__':
    assert part_one(test) == 136
    print(part_one(inputs))