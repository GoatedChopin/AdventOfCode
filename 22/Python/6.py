from collections import deque

with open("inputs/6.txt") as file:
    inputs = [i for i in file.readline()]


def part_one(inputs):
    queue = deque()
    for i, c in enumerate(inputs):
        queue.appendleft(c)
        if len(queue) > 4:
            queue.pop()
        if len(set(queue)) == 4:
            return i + 1


def part_two(inputs):
    queue = deque()
    for i, c in enumerate(inputs):
        queue.appendleft(c)
        if len(queue) > 14:
            queue.pop()
        if len(set(queue)) == 14:
            return i + 1

if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))