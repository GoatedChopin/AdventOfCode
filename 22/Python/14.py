from itertools import product

with open("inputs/14.txt") as file:
    lines = [line.replace("\n", "").split(" -> ") for line in file.readlines()]
    lines = [[eval("({})".format(tup)) for tup in line] for line in lines]

start = (500,0)

def draw_lines(lines):
    rocks = set()
    for line in lines:
        for i in range(len(line)-1):
            start, end = line[i], line[i+1]
            xs = sorted([start[0], end[0]])
            ys = sorted([start[1], end[1]])
            for coord in product(range(xs[0], xs[1]+1), range(ys[0], ys[1]+1)):
                rocks.add(coord)
    return rocks
    

def part_one(bound=173):  # dimensions are (562, 173)
    global start

    rocks = draw_lines(lines)
    print(rocks)
    num_rocks = len(rocks)

    def drop_sand():
        position = start
        while True:
            x, y = position
            down = x, y + 1
            left_down = x - 1, y + 1
            right_down = x + 1, y + 1

            if y == bound:  # we're done
                return True
            elif down not in rocks:
                position = down
            elif left_down not in rocks:
                position = left_down
            elif right_down not in rocks:
                position = right_down
            else:
                print(position)
                rocks.add(position)
                return False

    done = False
    while not done:
        done = drop_sand()

    return len(rocks) - num_rocks


def part_two(bound=174):  # dimensions are (562, 173)
    global start

    rocks = draw_lines(lines)
    print(rocks)
    num_rocks = len(rocks)

    def drop_sand():
        position = start
        while True:
            x, y = position
            down = x, y + 1
            left_down = x - 1, y + 1
            right_down = x + 1, y + 1

            if y == bound:  # we've hit the floor
                rocks.add(position)
                return False
            elif down not in rocks:
                position = down
            elif left_down not in rocks:
                position = left_down
            elif right_down not in rocks:
                position = right_down
            else:
                rocks.add(position)
                if position == start:
                    return True
                return False

    done = False
    while not done:
        done = drop_sand()

    return len(rocks) - num_rocks



if __name__ == "__main__":
    print(part_one())
    print(part_two())