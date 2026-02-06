from pprint import pprint
from common import standard_inputs


inputs = standard_inputs(8, True, True)


def percent_z(lis):
    return len([lis for i in lis if i.endswith('Z')]) / len(lis)


def process_inputs(inputs):
    directions = [c for c in inputs[0]]
    graph = {}
    for line in inputs[1:]:
        is_start, is_end = False, False
        if line == inputs[1]:
            is_start = True
        elif line == inputs[-1]:
            is_end = True
        line = line.replace(' ', '').replace('(', '').replace(')', '')
        start, left_right = line.split('=')
        left, right = left_right.split(',')
        graph[start] = (left, right)
        if is_start:
            graph['start'] = start
        elif is_end:
            graph['end'] = start
    return directions, graph


test = ['RL',
        'AAA = (BBB, CCC)',
        'BBB = (DDD, EEE)',
        'CCC = (ZZZ, GGG)',
        'DDD = (DDD, DDD)',
        'EEE = (EEE, EEE)',
        'GGG = (GGG, GGG)',
        'ZZZ = (ZZZ, ZZZ)']

test_2 = ['LLR',
          'AAA = (BBB, BBB)',
          'BBB = (AAA, ZZZ)',
          'ZZZ = (ZZZ, ZZZ)']


def part_one(inputs):
    directions, graph = process_inputs(inputs)
    direction_index = 0
    steps = 0
    current_position = 'AAA'
    while current_position != 'ZZZ':
        steps += 1
        if direction_index == len(directions):
            direction_index = 0
        if directions[direction_index] == 'L':
            print(f'Turning left from {current_position} to {graph[current_position][0]}')
            current_position = graph[current_position][0]
        elif directions[direction_index] == 'R':
            print(f'Turning right from {current_position} to {graph[current_position][1]}')
            current_position = graph[current_position][1]
        direction_index += 1
    return steps


def part_two(inputs):
    directions, graph = process_inputs(inputs)
    direction_index = 0
    steps = 0
    current_positions = []
    for position in graph.keys():
        if position.endswith('A'):
            current_positions.append(position)
    print(current_positions)
    while not (current_zs := percent_z(current_positions)) == 1.0:
        steps += 1
        if current_zs != 0.0:
            print(f'Step: {steps} | Percent Z is {percent_z(current_positions)}')
        if direction_index == len(directions):
            direction_index = 0
        if directions[direction_index] == 'L':
            for i in range(len(current_positions)):
                current_positions[i] = graph[current_positions[i]][0]
        elif directions[direction_index] == 'R':
            for i in range(len(current_positions)):
                current_positions[i] = graph[current_positions[i]][1]
        direction_index += 1
    return steps


if __name__ == '__main__':
    assert part_one(test) == 2
    assert part_one(test_2) == 6
    print(part_one(inputs))
    print(part_two(inputs))
