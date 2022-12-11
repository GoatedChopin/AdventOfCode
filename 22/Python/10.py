with open("inputs/10.txt") as file:
    inputs = [line.replace("\n", "") for line in file.readlines()]


def process_command(command):
    delta, cycles = 0, 0
    if command.startswith("noop"):
        cycles = 1
    else:
        delta = int(command.split(" ")[1])
        cycles = 2
    return delta, cycles


def part_one(inputs):
    X = 1
    cycle = 0

    key_cycles = {key: 0 for key in range(20, 241, 40)}

    i = 0
    while i < len(inputs):
        delta, cycles = process_command(inputs[i])
        for _ in range(cycles):
            cycle += 1
            if cycle in key_cycles:
                key_cycles[cycle] = X
        X += delta
        i += 1
    return sum([key*val for key, val in key_cycles.items()])


def sanity_check():
    with open("inputs/10.test") as file:
        test_inputs = [line.replace("\n", "") for line in file]
    assert part_one(test_inputs) == 13140


def part_two(inputs):
    X = 1
    cycle = 0

    key_cycles = set(range(41, 241, 40))
    level = 0
    i = 0
    cursor = 0
    screen = ""
    while i < len(inputs):
        delta, cycles = process_command(inputs[i])
        for _ in range(cycles):
            cycle += 1
            cursor += 1
            if cycle in key_cycles:
                cursor = 0
                screen += "\n"

            if cursor in (X-1, X, X+1):
                screen += "#"
            else:
                screen += "."
            
        X += delta
        i += 1
    return screen

if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))