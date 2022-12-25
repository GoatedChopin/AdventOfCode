import os
from collections import deque


def load_elves():
    with open("inputs" + os.sep + "23.txt") as file:
        inputs = file.readlines()
        inputs = [[c for c in line if c != "\n"] for line in inputs]
    elves = set()
    for row in range(len(inputs)):
        for col in range(len(inputs[0])):
            if inputs[row][col] == "#":
                elves.add((row, col))
    return elves


def shift(coord, delta):
    row, col = coord
    rd, cd = delta
    return row + rd, col + cd


def desired_destination(elf_coord, round=0):
    global elves

    movements = deque([(-1, 0), (1, 0), (0, -1), (0, 1)])
    movements.rotate(-round)
    
    row, col = elf_coord
    north_neighbors = 0
    south_neighbors = 0
    west_neighbors = 0
    east_neighbors = 0
    for r in range(row-1, row+2):
        for c in range(col-1, col+2):
            if (r, c) in elves:
                if r == row - 1:
                    north_neighbors += 1
                if r == row + 1:
                    south_neighbors += 1
                if c == col - 1:
                    west_neighbors += 1
                if c == col + 1:
                    east_neighbors += 1
    if sum([north_neighbors, south_neighbors, west_neighbors, east_neighbors]) == 0:
        return elf_coord


    neighbor_deque = deque([north_neighbors, south_neighbors, west_neighbors, east_neighbors])
    neighbor_deque.rotate(-round)


    for i in range(4):
        if neighbor_deque[i] == 0:
            return shift(elf_coord, movements[i])
    return elf_coord


def round(elves, id=0):

    blocked_tiles, open_tiles = 0, 0

    new_elves = set()
    proposed_positions = {}  # elf -> proposed_move
    proposal_counts = {}  # proposed_move -> int
    for elf in elves:
        proposed_move = desired_destination(elf, id)
        proposed_positions[elf] = proposed_move
        if proposed_move not in proposal_counts:
            proposal_counts[proposed_move] = 0
        proposal_counts[proposed_move] += 1
    for elf in elves:
        proposed_move = proposed_positions[elf]
        if proposal_counts[proposed_move] == 1:
            open_tiles += 1
            new_elves.add(proposed_move)
        else:
            blocked_tiles += 1
            new_elves.add(elf)
    # print("{}% of positions were open".format(100*open_tiles/sum([open_tiles, blocked_tiles])))
    return new_elves


def smallest_rectangle_empty_tiles(elves):
    min_row, max_row = float("inf"), -float("inf")
    min_col, max_col = float("inf"), -float("inf")
    for elf in elves:
        row, col = elf
        if row < min_row:
            min_row = row
        if row > max_row:
            max_row = row
        if col < min_col:
            min_col = col
        if col > max_col:
            max_col = col

    print("Going from {} to {}, {} to {}".format(min_row, max_row, min_col, max_col))
    empty_tiles = 0
    for row in range(min_row, max_row + 1):
        for col in range(min_col, max_col + 1):
            if (row, col) not in elves:
                empty_tiles += 1
    
    return empty_tiles


def part_one():
    global elves
    for id in range(10):
        elves = round(elves, id)
    empty_tiles = smallest_rectangle_empty_tiles(elves)
    return empty_tiles


def part_two():
    global elves
    new_elves = elves
    round_id = 0
    while True:
        new_elves = round(elves, round_id)
        round_id += 1
        if new_elves == elves:
            break
        elves = new_elves
    return round_id


if __name__ == "__main__":
    elves = load_elves()
    print(part_one())
    elves = load_elves()
    print(part_two())