import os


with open("inputs" + os.sep + "23.txt") as file:
    inputs = file.readlines()
    inputs = [[c for c in line if c != "\n"] for line in inputs]


elves = set()
for row in range(len(inputs)):
    for col in range(len(inputs[0])):
        if inputs[row][col] == "#":
            elves.add((row, col))


def desired_destination(elf_coord):
    global elves
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
    elif north_neighbors == 0:
        return (row - 1, col)
    elif south_neighbors == 0:
        return (row + 1, col)
    elif west_neighbors == 0:
        return (row, col - 1)
    elif east_neighbors == 0:
        return (row, col + 1)
    else:
        return elf_coord


def round(elves):

    blocked_tiles, open_tiles = 0, 0

    new_elves = set()
    proposed_positions = {}  # elf -> proposed_move
    proposal_counts = {}  # proposed_move -> int
    for elf in elves:
        proposed_move = desired_destination(elf)
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
    print("{}% of positions were open".format(100*open_tiles/sum([open_tiles, blocked_tiles])))
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
    for _ in range(10):
        elves = round(elves)
    empty_tiles = smallest_rectangle_empty_tiles(elves)
    return empty_tiles


if __name__ == "__main__":
    print(part_one())