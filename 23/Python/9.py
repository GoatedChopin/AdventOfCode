from pprint import pprint

from common import standard_inputs


inputs = standard_inputs(9, True, True)


test = ['0 3 6 9 12 15',
        '1 3 6 10 15 21',
        '10 13 16 21 30 45']


def get_diffs(history):
    return [history[i+1]-history[i] for i in range(len(history)-1)]


def is_complete(history):
    return len(history) == len([i for i in history if i == 0])


def process_inputs(inputs):
    histories = []
    for line in inputs:
        histories.append([int(i) for i in line.strip().split(' ')])
    return histories


def next_num(history):
    pyramid = [history]
    while not is_complete(pyramid[-1]):
        pyramid.append(get_diffs(pyramid[-1]))
    print(pyramid, end=' ')
    current = [0]
    while len(pyramid) > 1:
        current = pyramid.pop()
        next_num = current[-1] + pyramid[-1][-1]
        print(next_num, end=' ')
        pyramid[-1].append(next_num)
    return next_num


def prev_num(history):
    pyramid = [history]
    while not is_complete(pyramid[-1]):
        pyramid.append(get_diffs(pyramid[-1]))
    print(pyramid, end=' ')
    current = [0]
    while len(pyramid) > 1:
        current = pyramid.pop()
        prev_num = pyramid[-1][0] - current[0]
        print(prev_num, end=' ')
        pyramid[-1] = [prev_num] + pyramid[-1]
    return prev_num


def part_one(inputs):
    histories = process_inputs(inputs)
    total = 0
    for h in histories:
        total += next_num(h)
        print()
    return total


def part_two(inputs):
    histories = process_inputs(inputs)
    total = 0
    for h in histories:
        total += prev_num(h)
        print()
    return total


if __name__ == '__main__':
    assert part_one(test) == 114
    print(part_one(inputs))
    assert part_two(test) == 2
    print(part_two(inputs))
