with open("inputs/3.txt") as file:
    inputs = [i.replace("\n", "") for i in file.readlines()]


scores = {}

for i in range(65, 91):  # Uppercase letters Unicode indexes
    scores[chr(i)] = i - 38

for i in range(97, 123):  # Lowercase letters Unicode indexes
    scores[chr(i)] = i - 96


def part_one(inputs):
    score = 0
    for i in inputs:
        num_items = len(i)
        first, second = i[:num_items//2], i[num_items//2:]
        shared = set(first).intersection(set(second)).pop()
        score += scores[shared]
    return score

        
def part_two(inputs):
    score = 0
    for i in range(2, len(inputs), 3):
        first, second, third = inputs[i-2], inputs[i-1], inputs[i]
        badge = set(first).intersection(set(second)).intersection(set(third)).pop()
        score += scores[badge]
    return score

if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))