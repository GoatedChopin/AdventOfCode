with open("inputs/8.txt") as file:
    inputs = file.readlines()
    inputs = [[int(i) for i in line if i != "\n"] for line in inputs]
    shape = len(inputs), len(inputs[0])


def on_edge(position, shape):
    return 0 in position or position[0] == shape[0]-1 or position[1] == shape[1]-1


def next_position(current_position, delta):
    return current_position[0]+delta[0], current_position[1]+delta[1]


def visible_from(inputs, shape, original_height, current_position, delta):
    if on_edge(current_position, shape) and original_height > inputs[current_position[0]][current_position[1]]:
        return True

    elif original_height > inputs[current_position[0]][current_position[1]]:
        if visible_from(inputs, shape, original_height, next_position(current_position, delta), delta):
            return True
    
    return False
    

def part_one(inputs, shape):
    visible_trees = 0
    for row in range(shape[0]):
        for col in range(shape[1]):
            if on_edge((row, col), shape):
                visible_trees += 1
            else:
                height = inputs[row][col]
                for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)]:  # Up, Right, Down, Left
                    if visible_from(inputs, shape, height, next_position((row, col), direction), direction):
                        visible_trees += 1
                        break
    return visible_trees


def sanity_check(trees):
    assert part_one(trees, (5,5)) == 21


def scenic_score(dirs):
    score = 1
    for d in dirs:
        score *= d
    return score


def in_bounds(shape, position):  # assumes square input
    if -1 in position or shape[0] in position:
        return False
    return True
    

def view_length(tree, view):
    view_length = 0
    for v in view:
        view_length += 1
        if v >= tree:
            break
    return view_length


def build_view(inputs, shape, position, delta):
    view = []
    forward = next_position(position, delta)
    while in_bounds(shape, forward):
        view.append(inputs[forward[0]][forward[1]])
        forward = next_position(forward, delta)
    return view


def part_two(inputs, shape):
    top_score = 0
    for row in range(1, shape[0]-1):
        for col in range(1, shape[1]-1):
            scores = []
            for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)]:  # Up, Right, Down, Left
                view = build_view(inputs, shape, (row, col), direction)
                scores.append(view_length(inputs[row][col], view))
            top_score = max(top_score, scenic_score(scores))
    return top_score


if __name__ == "__main__":
    print(part_one(inputs, shape))
    print(part_two(inputs, shape))
