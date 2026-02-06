from collections import deque
from heapq import heappush, heappop

from common import standard_inputs, move, in_bounds, lookup


inputs = standard_inputs(11, True, True)
test = [
    '...#......',
    '.......#..',
    '#.........',
    '..........',
    '......#...',
    '.#........',
    '.........#',
    '..........',
    '.......#..',
    '#...#.....'
]

LEFT = (0, -1)
RIGHT = (0, 1)
UP = (-1, 0)
DOWN = (1, 0)


def prettify_galaxies(galaxies):
    return '\n'.join([''.join(galaxies[row]) for row in range(len(galaxies))])


def process_inputs(inputs, fold=True):
    galaxies = [[c for c in line] for line in inputs]
    blank_rows = []
    blank_cols = []
    for row in range(len(galaxies)):
        if sum([c == '#' for c in galaxies[row]]) == 0:
            blank_rows.append(row)
    for col in range(len(galaxies[0])):
        if sum([galaxies[row][col] == '#' for row in range(len(galaxies))]) == 0:
            blank_cols.append(col)

    if not fold:
        return galaxies, blank_rows, blank_cols

    for col in blank_cols[::-1]:
        galaxies = [
            galaxies[row][:col] + ['.'] + galaxies[row][col:] for row in range(len(galaxies))
        ]
    blank_row = ['.']*len(galaxies[0])
    for row in blank_rows[::-1]:
        galaxies = galaxies[:row] + [blank_row] + galaxies[row:]

    return galaxies


def part_one(inputs):
    galaxies = process_inputs(inputs)
    nodes = set()
    paths = {}
    for r in range(len(galaxies)):
        for c in range(len(galaxies[0])):
            if galaxies[r][c] == '#':
                nodes.add((r, c))
                paths[(r, c)] = {(r, c): 0}

    num_nodes = len(nodes)
    for node in nodes:
        if len(paths[node].keys()) == num_nodes:
            continue

        visited = set()
        visited_nodes = set()
        queue = deque([(node, 0)])
        while len(visited_nodes) < num_nodes:
            position, steps = queue.pop()
            for direction in [UP, DOWN, LEFT, RIGHT]:
                new_position = move(position, direction)
                if in_bounds(new_position, galaxies) and new_position not in visited:
                    visited.add(new_position)
                    if new_position in nodes:
                        visited_nodes.add(new_position)
                        paths[node][new_position] = min(paths[node].get(new_position, float('inf')), steps+1)
                        paths[new_position][node] = min(paths[node].get(new_position, float('inf')), steps+1)
                    queue.appendleft((new_position, steps + 1))

    total = 0
    bidirectional_visits = set()
    for node, path_dict in paths.items():
        for other_node, path_len in path_dict.items():
            if (node, other_node) not in bidirectional_visits:
                bidirectional_visits.add((node, other_node))
                bidirectional_visits.add((other_node, node))
                total += path_len
    return total


def part_two(inputs, num_folds=1000000):
    galaxies, blank_rows, blank_cols = process_inputs(inputs, fold=False)
    nodes = set()
    paths = {}
    for r in range(len(galaxies)):
        for c in range(len(galaxies[0])):
            if galaxies[r][c] == '#':
                nodes.add((r, c))
                paths[(r, c)] = {(r, c): 0}

    num_nodes = len(nodes)
    for node in nodes:
        if len(paths[node].keys()) == num_nodes:
            continue

        visited = set()
        visited_nodes = set()
        # queue = deque([(node, 0)])
        heap = [(0, node)]
        while len(visited_nodes) < num_nodes:
            steps, position = heappop(heap)
            for direction in [UP, DOWN, LEFT, RIGHT]:
                new_position = move(position, direction)
                if new_position[0] in blank_rows or new_position[1] in blank_cols:
                    step_delta = num_folds
                else:
                    step_delta = 1
                if in_bounds(new_position, galaxies) and new_position not in visited:
                    visited.add(new_position)
                    if new_position in nodes:
                        visited_nodes.add(new_position)
                        paths[node][new_position] = min(paths[node].get(new_position, float('inf')), steps+step_delta)
                        paths[new_position][node] = min(paths[node].get(new_position, float('inf')), steps+step_delta)
                    heappush(heap, (steps + step_delta, new_position))

    total = 0
    bidirectional_visits = set()
    for node, path_dict in paths.items():
        for other_node, path_len in path_dict.items():
            if (node, other_node) not in bidirectional_visits:
                bidirectional_visits.add((node, other_node))
                bidirectional_visits.add((other_node, node))
                total += path_len
    return total


if __name__ == '__main__':
    assert part_one(test) == 374
    # print(part_one(inputs))
    print(part_two(inputs))
