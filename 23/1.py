from common import standard_inputs


inputs = standard_inputs(1, True)
DIGITS = {'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'}
SPELLED = {'one': '1', 'two': '2',
           'three': '3', 'four': '4',
           'five': '5', 'six': '6',
           'seven': '7', 'eight': '8',
           'nine': '9', 'zero': '0'}


def all_digits(line):
    return [c for c in line if c in DIGITS]


def part_one(inputs):
    total = 0
    for line in inputs:
        digits = all_digits(line)
        total += int(digits[0] + digits[-1])
    return total


def all_spelled(line, num):
    i = 0
    matches = []
    try:
        while True:
            matches.append((line.index(num, i), SPELLED[num]))
            i = matches[-1][0] + 1
    except ValueError:
        return matches
    while num in line[i:]:
        print(line[i:])
        match = line[i:].index(num)
        matches.append((match, SPELLED[num]))
        i = match + len(num)
    return matches


def indexed_digits(line):
    matches = [(i, c) for i, c in enumerate(line) if c in DIGITS]
    for num in SPELLED.keys():
        # print(num)
        matches += all_spelled(line, num)
    return sorted(matches)


def part_two(inputs):
    total = 0
    print(len(inputs))
    for line in inputs:
        # print(line)
        digits = indexed_digits(line)
        total += int(digits[0][-1] + digits[-1][-1])
    return total


if __name__ == '__main__':
    print(part_one(inputs))
    print(part_two(inputs))
