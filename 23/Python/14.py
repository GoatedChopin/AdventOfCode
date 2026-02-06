from datetime import datetime
from functools import lru_cache

import numpy as np
import xxhash 

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


class HashableArray(list):
    def __init__(self, array):
        self.array = np.array(array)
        self.hasher = xxhash.xxh64()
    
    def __hash__(self) -> int:
        self.hasher.update(self.array.data.tobytes())
        h = self.hasher.intdigest()
        self.hasher.reset()
        return h

    def __getitem__(self, key):
        return self.array[key]

    def __setitem__(self, key, item):
        self.array[key] = item

    def __len__(self):
        return len(self.array)


def process_inputs(inputs):
    return HashableArray([[c for c in line] for line in inputs])


@lru_cache() # maxsize=1000000
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


@lru_cache() # maxsize=1000
def score_grid(grid):
    total = 0
    val = len(grid)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if lookup((row, col), grid) == 'O':
                total += val
        val -= 1
    return total


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


# def part_two(inputs):
#     grid = process_inputs(inputs)
#     print(datetime.now())
#     one_percent = 1000000000 // 100
#     for i in range(1000000000):
#         if i % one_percent == 0:
#             print(f'{i}: {datetime.now()}')
#         for dir in range(4):
#             moved = True
#             while moved:
#                 grid, moved = grid_step(grid)
#             grid = np.rot90(grid, -1)
#         grid = np.rot90(grid, -1)
#     total = 0
#     val = len(grid)
#     for row in range(len(grid)):
#         for col in range(len(grid[0])):
#             if lookup((row, col), grid) == 'O':
#                 total += val
#         val -= 1
#     return total


def part_two(inputs):
    grid = process_inputs(inputs)
    scores = []
    with open('14.scores', 'w') as file:
        file.write('[')
    for i in range(500):
        print(f'{i}: {datetime.now()}')
        for dir in range(4):
            moved = True
            while moved:
                grid, moved = grid_step(grid)
            grid.array = np.rot90(grid.array, -1)
        scores.append((i, score_grid(grid)))
        with open('14.scores', 'a') as file:
            file.write(f'{scores[-1]}, ')
    with open('14.scores', 'a') as file:
        file.write(']')
    breakpoint()


def is_cycle(history, cycle_len=1):
    cycle = history[:cycle_len]
    print(f"Testing cycle: {cycle}")
    for i in range(cycle_len, len(history), cycle_len):
        if len(history) > i + cycle_len and history[i:i+cycle_len] != cycle:
            return False
        elif len(history) <= i + cycle_len:
            hist_i = i
            cycle_i = 0
            while hist_i < len(history):
                if history[hist_i] != cycle[cycle_i]:
                    return False
                cycle_i += 1
                hist_i += 1
    return True


def find_cycle(history):
    cycle_len = 1
    while not is_cycle(history=history, cycle_len=cycle_len):
        cycle_len += 1
        if cycle_len == len(history):
            raise ValueError('Cycle does not exist')
    return cycle_len


if __name__ == '__main__':
    assert part_one(test) == 136
    print(part_two(inputs))
