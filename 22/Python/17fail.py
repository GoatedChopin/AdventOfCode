with open("inputs/17.txt") as file:
    inputs = [i for i in file.readline()]

rocks = [
"####",  # flat line

""".#.
###
.#.""",  # plus

"""..#
..#
###""",  # L shape

"""#
#
#
#""",  # vertical line

"""##
##"""  # box
]

rocks = [
    [[1,1,1,1]],

    [[0,1,0],
     [1,1,1],
     [0,1,0]],

    [[0,0,1],
     [0,0,1],
     [1,1,1]],

    [[1,1],
     [1,1]]
]

tetris = [[0 for _ in range(7)] for row in range(6600)]  # 2022 rocks, average height of 13/4


def gas_delta(gas_char, rock_shape, tetris, start_row, start_col, bound=6):
    match gas_char:
        case ">":
            if len(rock_shape[0]) + start_col - 1 < 6:
                for row in range(len(rock_shape)):
                    for col in range(len(rock_shape[0])):
                        if rock_shape[row][col] == 1 and tetris[start_row + row][start_col + col + 1] == 1:  # cannot move right, shape is blocked.
                            return 0
                return 1
        case "<":
            if start_col > 0:
                for row in range(len(rock_shape)):
                    for col in range(len(rock_shape[0])):
                        if rock_shape[row][col] == 1 and tetris[start_row + row][start_col + col - 1] == 1:  # cannot move right, shape is blocked.
                            return 0
                return -1
    return 0


def drop_rock(rock_shape, tetris, gas_movements, gas_index, highest_rock_index):
    stopped = False
    start_row = highest_rock_index - (3 + len(rock_shape))
    start_col = 2
    while not stopped:
        gas = gas_delta(gas_movements[gas_index], rock_shape, tetris, start_row, start_col)
        gas_index += 1
        if gas_index == len(gas_movements):
            gas_index = 0
        start_col += gas
        
        # move with gas
        for row in range(len(rock_shape)):
            for col in range(len(rock_shape[0])):
                if rock_shape[row][col] == 1:  # Part of shape is solid rock
                    if start_row + row + 1 == len(tetris) or tetris[start_row + row + 1][start_col + col] == 1: # Block underneath shape is solid rock
                        stopped = True
        if not stopped:
            start_row += 1

    def add_rock(rock_shape, start_row, start_col):
        for row in range(len(rock_shape)):
            for col in range(len(rock_shape[0])):
                if rock_shape[row][col] == 1:
                    tetris[start_row + row][start_col + col] = 1


    gas = gas_delta(gas_movements[gas_index], rock_shape, tetris, start_row, start_col)
    gas_index += 1
    if gas_index == len(gas_movements):
        gas_index = 0
    start_col += gas

    add_rock(rock_shape, start_row, start_col)
    return tetris, start_row-1, gas_index


def part_one(gas_movements, debug=True):
    global tetris
    rock_index = 0
    gas_index = 0
    start_row = len(tetris)+1
    for i in range(2022):
        tetris, next_start_row, gas_index = drop_rock(rocks[rock_index], tetris, gas_movements, gas_index, start_row)
        start_row = min(start_row, next_start_row)
        if debug:
            for r in range(start_row, len(tetris)):
                print(tetris[r])
            if input() == "exit":
                break
        rock_index += 1
        if rock_index == len(rocks):
            rock_index = 0
    return tetris


if __name__ == "__main__":
    test_inputs = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    tetris = part_one(test_inputs, debug=False)
    empty_rows = 0
    while 1 not in tetris[empty_rows]:
        empty_rows += 1

    for t in tetris[empty_rows:]:
        assert 1 in t
        print(t)
    print(len(tetris) - empty_rows)