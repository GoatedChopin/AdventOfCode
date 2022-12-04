with open("inputs/4.txt") as file:
    inputs = file.readlines()

comma_sep = lambda line: line.split(",")
dash_sep = lambda assignment: [int(i) for i in assignment.split("-")]

for i, line in enumerate(inputs):
    inputs[i] = comma_sep(line)
    for elf, assignment in enumerate(inputs[i]):
        inputs[i][elf] = dash_sep(assignment)


def contains(elf1, elf2):  # Should be read, elf1 contains elf2
    if elf1[0] <= elf2[0] and elf1[1] >= elf2[1]:
        return True
    return False


def part_one(inputs):
    contained = 0
    for elf1, elf2 in inputs:
        if contains(elf1, elf2) or contains(elf2, elf1):
            contained += 1
    return contained


def overlap(elf1, elf2):
    if (elf1[0] <= elf2[0] and elf1[1] >= elf2[0]) or (elf2[0] <= elf1[0] and elf2[1] >= elf1[0]):
        return True
    return False

assert not overlap([56, 56], [57, 70])
assert overlap([56, 62], [51, 59])

def part_two(inputs):
    return sum(overlap(elf1, elf2) for elf1, elf2 in inputs)


if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))
