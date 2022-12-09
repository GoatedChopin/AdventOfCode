with open("inputs/9.txt") as file:
    inputs = [i.split(" ") for i in file.readlines()]
    inputs = [(dir_char, int(num)) for dir_char, num in inputs]


dirs = {"D": (1, 0), "R": (0, 1), "U": (-1, 0), "L": (0, -1)}


def delta(head, tail):
    return head[0]-tail[0], head[1]-tail[1]


def unitize(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0


def floor(vector):
    return tuple((unitize(i) for i in vector))


def adjacent(head, tail):
    return abs(head[0]-tail[0]) <= 1 and abs(head[1]-tail[1]) <= 1


def move(knot, direction):
    return knot[0]+direction[0], knot[1]+direction[1]


def part_one(inputs):
    global dirs
    head, tail = (0, 0), (0, 0)
    visited_set = set()
    visited_set.add(tail)
    for dir_char, reps in inputs:
        direction = dirs[dir_char]
        for _ in range(reps):
            head = move(head, direction)
            if not adjacent(head, tail):
                move_dir = floor(delta(head, tail))
                print("Head is at {}, tail is at {}, moving by {}".format(head, tail, move_dir))
                tail = move(tail, move_dir)
                visited_set.add(tail)
    return len(visited_set)


def part_two(inputs):
    global dirs
    knot_paths = {i:[(0, 0)] for i in range(10)}
    knots = [(0, 0) for _ in range(10)]
    for dir_char, reps in inputs:
        direction = dirs[dir_char]
        for _ in range(reps):
            knots[0] = move(knots[0], direction)
            knot_paths[0].append(knots[0])
    for ki in range(1,10):
        for position in knot_paths[ki-1]:
            if not adjacent(position, knots[ki]):
                move_dir = floor(delta(position, knots[ki]))
                knots[ki] = move(knots[ki], move_dir)
                knot_paths[ki].append(knots[ki])
    return len(set(knot_paths[9]))


def sanity_check():
    test_inputs = [("R", 5), ("U", 8), ("L", 8), ("D", 3), ("R", 17), ("D", 10), ("L", 25), ("U", 20)]
    print(part_two(test_inputs))  # Should be 36


if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))
