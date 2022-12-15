with open("inputs/14.txt") as file:
    lines = [line.replace("\n", "").split(" -> ") for line in file.readlines()]
    lines = [[eval("({})".format(tup)) for tup in line] for line in lines]


def largest_dimensions(lines):
    x, y = 0, 0
    for line in lines:
        x = max(x, max([coord[0] for coord in line]))
        y = max(y, max([coord[1] for coord in line]))
    return x, y


def get_delta(start, end):
    rd, cd = end[0] - start[0], end[1] - start[1]

    if rd > 0:
        rd = 1
    elif rd < 0:
        rd = -1
    else:
        rd = 1
    
    if cd > 0:
        cd = 1
    elif cd < 0:
        cd = -1
    else:
        cd = 1

    return rd, cd


def bring_down(coord):
    return coord[0]-1, coord[1]-1


def draw_lines(lines):
    rowdim, coldim = largest_dimensions(lines)
    # print(rowdim, coldim)
    matrix = [["." for c in range(rowdim)] for r in range(coldim)]

    # matrix[0][500] = "*"

    for line in lines:
        for i in range(len(line)-1):
            start, end = bring_down(line[i]), bring_down(line[i+1])
            r_delta, c_delta = get_delta(start, end)
            r_comp, c_comp = 1 if r_delta > 0 else -1, 1 if c_delta > 0 else -1
            for r in range(start[0], end[0]+1, r_delta):
                for c in range(start[1], end[1]+r_comp, c_delta+c_comp):
                    # print(r, c)
                    matrix[c][r] = "#"
    
    return matrix


def show_matrix(matrix, skiprows=0, skipcols=450, stoprows=0, stopcols=0, highlights=[]):
    for hrow, hcol in highlights:
        matrix[hrow][hcol] = "X"
    for r in range(skiprows, len(matrix)-stoprows):
        for c in range(skipcols, len(matrix[0])-stopcols):
            print(matrix[r][c], end="")
        print()


def sand_step(matrix, coord):
    if coord[0] == len(matrix)-1:
        return (-1, 0)

    row, col = coord
    left_wall = coord[1] == 0 # or matrix[row][col-1] in ("#", "O")
    right_wall = coord[1] == len(matrix[0])-1 # or matrix[row][col+1] in ("#", "O")


    if matrix[row+1][col] in (".", "+"):  # empty spot beneath, sand falls
        return (row + 1, col)
    elif matrix[row+1][col] in ("#", "O"):  # spot is filled beneath, time to check diagonals
        if not left_wall and matrix[row+1][col-1] not in ("#", "O"):
            return (row + 1, col - 1)
        elif not right_wall and matrix[row+1][col+1] not in ("#", "O"):
            return (row + 1, col + 1)
    return row, col


def part_one(matrix, start=(0, 500), show=True):
    next_sand = None
    sitting_sand = 0
    while next_sand != (-1, 0):
        
        if show:
            show_matrix(matrix)

        if next_sand is not None:
            row, col = next_sand
            matrix[row][col] = "O"
            sitting_sand += 1

        sand = start
        next_sand = sand_step(matrix, sand)

        while next_sand != sand:
            sand = next_sand
            next_sand = sand_step(matrix, sand)

            if sand[0] == len(matrix)-1:
                break

            row, col = next_sand
            matrix[row][col] = "+"

        print("Sand stopped at {}".format(next_sand))

    return sitting_sand


if __name__ == "__main__":
    print(largest_dimensions(lines))
    matrix = draw_lines(lines)
    print(part_one(matrix))
    show_matrix(matrix)
    breakpoint()