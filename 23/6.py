from common import standard_inputs


inputs = standard_inputs(6, True, True)


test_inputs = ["Time:      7  15   30", "Distance:  9  40  200"]


def parse_inputs(inputs):
    for i in range(len(inputs)):
        while '  ' in inputs[i] or '\t' in inputs[i]:
            inputs[i] = inputs[i].strip().replace('  ', ' ').replace('\t', ' ')
    print(inputs)
    times = [int(i) for i in inputs[0].replace('Time: ', '').split(' ')]
    distances = [int(i) for i in inputs[1].replace('Distance: ', '').split(' ')]
    return times, distances


def total_distance(hold_time, race_time):
    return (race_time-hold_time)*hold_time


def product(x):
    p = 1
    for i in x:
        p *= i
    return p


def part_one(inputs):
    times, distances = parse_inputs(inputs)
    beats_record = [0]*len(times)

    race = 0
    for t, d in zip(times, distances):
        for i in range(1, t):
            if total_distance(i, t) > d:
                beats_record[race] += 1
        race += 1
    return product(beats_record)


def part_two(inputs):
    times, distances = parse_inputs(inputs)
    time = int(''.join([str(i) for i in times]))
    distance = int(''.join([str(i) for i in distances]))
    beats_record = 0
    for i in range(1, time):
        if total_distance(i, time) > distance:
            beats_record += 1
    return beats_record


if __name__ == '__main__':
    print(part_two(inputs))
