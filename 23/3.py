from collections import deque

from common import standard_inputs


inputs = standard_inputs(3, True)
NUMBERS = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}
NON_SYMBOLS = NUMBERS.union({'.'})


def all_symbols(inputs):
    symbols = []
    for row in range(len(inputs)):
        for col in range(len(inputs[0])):
            if inputs[row][col] not in NON_SYMBOLS:
                symbols.append((row, col))
    return symbols


def in_bounds(r, c):
    return 0 <= r < len(inputs) and 0 <= c < len(inputs[0])


def part_one(inputs):
    symbols = all_symbols(inputs)
    print(symbols)

    skips = set()
    schematic_numbers = []

    def process_number(r, c):
        number = deque()
        number.append(inputs[r][c])
        dc = 1
        while in_bounds(r, c+dc) and inputs[r][c+dc] in NUMBERS:
            number.append(inputs[r][c+dc])
            skips.add((r, c+dc))
            dc += 1
        dc = -1
        while in_bounds(r, c+dc) and inputs[r][c+dc] in NUMBERS:
            number.appendleft(inputs[r][c+dc])
            skips.add((r, c+dc))
            dc -= 1
        print(int(''.join(number)))
        schematic_numbers.append(int(''.join(number)))

    for row, col in symbols:
        for r in range(row-1, row+2):
            for c in range(col-1, col+2):
                if in_bounds(r, c) and (r, c) not in skips and inputs[r][c] in NUMBERS:
                    process_number(r, c)

    return sum(schematic_numbers)


def part_two(inputs):
    symbols = all_symbols(inputs)
    print(symbols)

    skips = set()
    total = 0

    def process_number(r, c):
        number = deque()
        number.append(inputs[r][c])
        dc = 1
        while in_bounds(r, c+dc) and inputs[r][c+dc] in NUMBERS:
            number.append(inputs[r][c+dc])
            skips.add((r, c+dc))
            dc += 1
        dc = -1
        while in_bounds(r, c+dc) and inputs[r][c+dc] in NUMBERS:
            number.appendleft(inputs[r][c+dc])
            skips.add((r, c+dc))
            dc -= 1
        return (int(''.join(number)))

    for row, col in symbols:
        if inputs[row][col] == '*':
            adjacent = []
            for r in range(row-1, row+2):
                for c in range(col-1, col+2):
                    if in_bounds(r, c) and (r, c) not in skips and inputs[r][c] in NUMBERS:
                        adjacent.append(process_number(r, c))
            if len(adjacent) == 2:
                total += (adjacent[0] * adjacent[1])

    return total


if __name__ == '__main__':
    print(part_one(inputs))
    print(part_two(inputs))
