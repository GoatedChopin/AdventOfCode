import os


def aggregate_nums(ins):
    new_ins = []
    current = ""
    for i in range(len(ins)):
        try:
            _ = int(ins[i])
            current = current + ins[i]
        except:
            if current:
                new_ins.append(int(current))
                current = ""
            new_ins.append(ins[i])
    return new_ins


def lookup(coordinate):
    global inputs
    row, column = coordinate
    return inputs[row][column]


def is_wall(coord):
    global inputs
    row, col = coord
    return inputs[row][col] == "#"


def shift(position, delta):
    row, column = position
    rd, cd = delta
    return row + rd, column + cd


def in_bounds(coord):
    global inputs
    global empty_set
    if coord in empty_set:
        return False
    row, col = coord
    return 0 <= row < len(inputs) and 0 <= col < len(inputs[0])


def out_of_bounds(coord):
    return not in_bounds(coord)


def first_last_coordinates(start_coord, direction):
    global inputs
    while lookup(start_coord) == " ":
        start_coord = shift(start_coord, direction)
    first = start_coord
    while in_bounds(shift(start_coord, direction)):
        start_coord = shift(start_coord, direction)
    last = start_coord
    return first, last


def wrap(next_position):
    global current_direction
    global current_position
    global first_last_map
    match current_direction:
        case (0, 1): # right
            row, _ = next_position
            wrapped_position = first_last_map[(row, 0)][0]  # wrap to the first position
            if not is_wall(wrapped_position):  # blocked
                return wrapped_position
            else:
                return current_position
        case (1, 0): # down
            _, col = next_position
            wrapped_position = first_last_map[(0, col)][0]  # wrap to the first position
            if is_wall(wrapped_position):
                return wrapped_position
            else:
                return current_position
        case (0, -1): # left
            row, _ = next_position
            wrapped_position = first_last_map[(row, 0)][1]  # wrap to the last position
            if is_wall(wrapped_position):  # blocked
                return wrapped_position
            else:
                return current_position        
        case (-1, 0): # up
            _, col = next_position
            wrapped_position = first_last_map[(0, col)][1]  # wrap to the last position
            if is_wall(wrapped_position):
                return wrapped_position
            else:
                return current_position
    

def move(new_direction):
    global current_position
    global current_direction
    number_moves = new_direction
    for _ in range(number_moves):
        next_position = shift(current_position, current_direction)
        if out_of_bounds(next_position):
            next_position = wrap(next_position)
        elif is_wall(next_position):
            break
        current_position = next_position


def turn(new_direction):
    global current_direction
    directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    d_i = directions.index(current_direction)

    match new_direction:
        case "L":
            d_i -= 1
            if d_i == -1:
                d_i == 3
        case "R":
            d_i += 1
            if d_i == 4:
                d_i = 0

    current_direction = directions[d_i] 


def take_direction(new_direction):
    direction_type = type(new_direction)
    if direction_type == int:
        move(new_direction)
    else:
        turn(new_direction)


def score_final_position():
    global current_direction
    global current_position
    row, col = current_position
    row_multiplier = 1000
    col_multiplier = 4
    facing = 0
    match current_direction:
        case (0, 1): facing = 0
        case (1, 0): facing = 1
        case (0, -1): facing = 2
        case (-1, 0): facing = 3
    return sum([row*row_multiplier, col*col_multiplier, facing])


if __name__ == "__main__":

    with open("inputs" + os.sep + "22.txt") as file:
        inputs = file.readlines()
        inputs = [[c for c in line if c != "\n"] for line in inputs]
        directions = aggregate_nums(inputs[-1])
        inputs = inputs[:-2]

    current_direction = (0, 1)
    current_position  = (0, 0)

    while lookup(current_position) != ".":
        row, column = current_position
        column += 1
        current_position = (row, column)

    empty_set = set()
    for row in range(len(inputs)):
        for col in range(len(inputs[0])):
            if col >= len(inputs[row]) or inputs[row][col] == " ":
                empty_set.add((row, col))


    first_last_map = {}
    for row in range(len(inputs)):
        for col in range(len(inputs[0])):
            if row == 0:
                first_last_map[(row, col)] = first_last_coordinates((row, col), (1, 0))  # down
            elif col == 0:
                first_last_map[(row, col)] = first_last_coordinates((row, col), (0, 1))  # right
    
    for direction in directions:
        take_direction(direction)
    
    print("Last position is {}".format(current_position))
    print(score_final_position())