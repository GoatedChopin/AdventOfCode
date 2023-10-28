from common import get_inputs


inputs = get_inputs(3)


def arrow_to_delta(char):
    match char:
        case '^': return (0, 1)
        case '>': return (1, 0)
        case 'v': return (0, -1)
        case '<': return (-1, 0)
    print(char)


def move(coord, delta):
    return coord[0] + delta[0], coord[1] + delta[1]


def part_one():
    position = (0, 0)
    visits = {position: 1}
    for char in inputs.strip():
        delta = arrow_to_delta(char)
        position = move(position, delta)
        if position not in visits:
            visits[position] = 0
        visits[position] += 1
    return len(visits.keys())


def part_two():
    real = (0, 0)
    robo = (0, 0)
    real_turn = True
    visits = {real: 1}
    for char in inputs.strip():
        delta = arrow_to_delta(char)
        if real_turn:
            real = move(real, delta)
            if real not in visits:
                visits[real] = 0
            visits[real] += 1
        else:
            robo = move(robo, delta)
            if robo not in visits:
                visits[robo] = 0
            visits[robo] += 1
        real_turn = not real_turn
    return len(visits.keys())


if __name__ == '__main__':
    print(part_one())
    print(part_two())