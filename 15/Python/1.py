from common import get_inputs


inputs = get_inputs(1)


def part_one():
    floor = 0
    for c in inputs:
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1
    return floor


def part_two():
    floor = 0
    for i, c in enumerate(inputs):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1
        if floor < 0:
            return i + 1


if __name__ == '__main__':
    print(part_one())
    print(part_two())