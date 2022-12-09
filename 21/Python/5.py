import time

inputs = []
row_dim = 0
col_dim = 0
with open("inputs/5.txt") as file:
    for _ in file:
        start, end = file.readline().replace("\n", "").split(" -> ")
        start, end = [int(i) for i in start.split(",")], [int(i) for i in end.split(",")]
        row_dim = max([row_dim, start[0]+1, end[0]+1])
        col_dim = max([col_dim, start[1]+1, end[1]+1])
        inputs.append((start, end))

matrix = [[0 for c in range(col_dim)] for r in range(row_dim)]

def show_matrix(test_mat):
    print("\n")
    for i in range(len(test_mat)):
        print(test_mat[i])


def delta(p2, p1):
    if p1[0]-p2[0] > 0:
        r = 1
    elif p1[0]-p2[0] < 0:
        r = -1
    else:
        r = 0

    if p1[1]-p2[1] > 0:
        c = 1
    elif p1[1]-p2[1] < 0:
        c = -1
    else:
        c = 0
    
    return r, c


def mutate(point, delta):
    return point[0]+delta[0], point[1]+delta[1]


def part_one(inputs, matrix, show=False):
    for start, end in inputs:
        print("{} -> {}".format(start, end))
        d = delta(start, end)
        # if 0 in d:  # Check if straight line
        print(d)
        if show:
            input()
        while start != mutate(end, d):
            matrix[start[0]][start[1]] += 1
            start = mutate(start, d)
            if show:
                show_matrix(matrix)
                time.sleep(0.5)

    out = 0
    for r in range(len(matrix)):
        for c in range(len(matrix[0])):
            if matrix[r][c] > 1:
                out += 1
    return out


def sanity_check():
    test = [((0,9), (5,9)), 
            ((8,0), (0,8)), 
            ((9,4), (3,4)),
            ((2,2), (2,1)),
            ((7,0), (7,4)),
            ((6,4), (2,0)),
            ((0,9), (2,9)),
            ((3,4), (1,4)),
            ((0,0), (8,8)),
            ((5,5), (8,2))]

    test_mat = [[0 for c in range(10)] for r in range(10)]
    part_one(test, test_mat)


def part_two(inputs):
    pass


if __name__ == "__main__":
    # sanity_check()
    print(part_one(inputs, matrix))