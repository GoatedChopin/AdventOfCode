import os
from collections import defaultdict, deque


with open("inputs" + os.sep + "24.test") as file:
    inputs = file.readlines()
    inputs = [[c for c in line if c != "\n"] for line in inputs]


storms = defaultdict(list)  # (row, col) -> [(1, 0), (0, -1) ... ]  (storm deltas)
storm_array = [[0 for _ in range(len(inputs[0]))] for _ in range(len(inputs))]


def storm_delta(character):
    match character:
        case "v": return (1, 0)
        case ">": return (0, 1)
        case "^": return (-1, 0)
        case "<": return (0, -1)
    return


for row in range(len(inputs)):
    for col in range(len(inputs)):
        delta = storm_delta(inputs[row][col])
        if delta:
            storms[(row, col)].append(delta)
            storm_array[row][col] += 1
        if 0 in (row, col) or row == len(inputs)-1 or col == len(inputs[0])-1:
            storm_array[row][col] = -float("inf")

storm_array[0][1] = 0
storm_array[-1][-2] = 0# [26][120] = 0


def shift(coord, delta):
    row, col = coord
    rd, cd = delta
    return row + rd, col + cd


def out_of_bounds(storm_array, coord):
    num_rows, num_cols = len(storm_array), len(storm_array[0])
    row, col = coord
    return not (0 <= row < num_rows and 0 <= col < num_cols)


def on_edge(storm_array, coord):
    row, col = coord
    return storm_array[row][col] < -1000


def wrap_around(storm_array, storm_coord, storm_delta):
    num_rows, num_cols = len(storm_array), len(storm_array[0])
    row, col = storm_coord
    match storm_delta:
        case (1, 0): return (1, col)
        case (0, 1): return (row, 1)
        case (-1, 0): return (num_rows-2, col)
        case (0, -1): return (row, num_cols-2)


def step(storms, storm_array):
    for row in range(len(storm_array)):
        for col in range(len(storm_array[0])):
            if storms[(row, col)] != []:
                for _ in range(len(storms[(row, col)])):
                    storm_array[row][col] -= 1
                    storm = storms[(row, col)].pop()
                    new_storm_coord = shift((row, col), storm)
                    # print("{}, {}: {}".format(row, col, storm))
                    # check to see if the new_storm_coord has reached the edge of the map
                    if out_of_bounds(storm_array, new_storm_coord) or on_edge(storm_array, new_storm_coord):
                        new_storm_coord = wrap_around(storm_array, new_storm_coord, storm)

                    new_row, new_col = new_storm_coord
                    storm_array[new_row][new_col] += 1
                    storms[(new_row, new_col)].append(storm)
    return storms, storm_array


def valid_neighbors(coord, next_storm_array):
    neighbor_deltas = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]
    neighbors = []
    for delta in neighbor_deltas:
        new_row, new_col = shift(coord, delta)
        if next_storm_array[new_row][new_col] == 0:
            neighbors.append((new_row, new_col))
    return neighbors


def bfs(start=(0, 1), end=(26, 120)):

    queue = deque()
    storms_timelapse = {0: storms}
    storm_array_timelapse = {0: storm_array}

    def get_storms(path_length):
        if path_length in storms_timelapse:
            return storms_timelapse[path_length]
        for i in range(1, path_length + 1):
            if i not in storms_timelapse:
                print("Reached path length of {}".format(i))
                next_storms, next_storm_array = step(storms_timelapse[i-1], storm_array_timelapse[i-1])
                storms_timelapse[i] = next_storms
                storm_array_timelapse[i] = next_storm_array

        return storm_array_timelapse[path_length]

    def bfs_step():
        path_length, coord = queue.pop()
        if coord == end:
            return path_length
        
        next_storm_array = get_storms(path_length + 1)
        for neighbor in valid_neighbors(coord, next_storm_array):
            queue.appendleft((path_length + 1, neighbor))

    queue.appendleft((0, start))
    while True:
        path_length = bfs_step()
        if path_length:
            return path_length


def part_one(start, end):
    global storms
    global storm_array
    print(bfs(start, end))


if __name__ == "__main__":
    # print(inputs[26][120])
    # print(storm_array[26][120])
    print(part_one((0, 1), (6, 5)))


