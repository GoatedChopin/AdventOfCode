from collections import deque

with open("inputs/5.txt") as file:
    lines = [i.replace("\n", "") for i in file.readlines()]

stacks = {i:deque() for i in range(1, 10)}

i = 0
line = lines[i]
while not line.startswith("move"):
    point = 0
    stack = 1
    while stack < 10:
        string_slice = line[point:point+4]
        if string_slice.startswith("["):
            stacks[stack].appendleft(string_slice[1])
        stack += 1
        point += 4
    i += 1
    line = lines[i]

lines = lines[i:]

def part_one(lines):
    for line in lines:
        s = line.split(" ")
        for _ in range(int(s[1])):
            stacks[int(s[5])].append(stacks[int(s[3])].pop())
    out = ""
    for i in range(1, 10):
        out = out + stacks[i].pop()
    return out

def part_two(lines):
    for line in lines:
        s = line.split(" ")
        for i in range(int(s[1])):
            stacks[int(s[5])].append(stacks[int(s[3])][len(stacks[int(s[3])])-int(s[1])+i])
        for _ in range(int(s[1])):
            stacks[int(s[3])].pop()
    out = ""
    for i in range(1, 10):
        out = out + stacks[i].pop()
    return out

if __name__ == "__main__":
    # print(part_one(lines))
    print(part_two(lines))