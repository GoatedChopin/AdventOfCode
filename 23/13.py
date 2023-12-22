from common import standard_inputs, in_bounds, lookup


inputs = standard_inputs(13, False, False)
test = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""


def process_inputs(inputs):
    return [[[c for c in line] for line in grid.split('\n')] for grid in inputs.split('\n\n')]


def is_mirror(grid, row=None, col=None, smudges=0, log=False):
    found_smudges = 0
    if row is not None:
        up, down = row, row-1
        while in_bounds((up, 0), grid) and in_bounds((down, 0), grid):
            for i in range(len(grid[0])):
                if not lookup((up, i), grid) == lookup((down, i), grid):
                    if log:
                        print(f"Row is not mirror: {row}")
                    found_smudges += 1
                    if found_smudges > smudges:
                        return False
            up -= 1
            down += 1
        if log:
            print(f"Row {row} is mirror? {found_smudges == smudges}")
        return True if found_smudges == smudges else False
    elif col is not None:
        left, right = col, col-1
        while in_bounds((0, left), grid) and in_bounds((0, right), grid):
            for i in range(len(grid)):
                if not lookup((i, left), grid) == lookup((i, right), grid):
                    if log:
                        print(f"Col is not mirror: {col}")
                    found_smudges += 1
                    if found_smudges > smudges:
                        return False
            left -= 1
            right += 1
        if log:
            print(f"Col {col} is mirror? {found_smudges == smudges}")
        return True if found_smudges == smudges else False


def part_one(inputs):
    total = 0
    grids = process_inputs(inputs)
    # breakpoint()
    for grid in grids:
        total += sum([r*100 for r in range(len(grid)) if is_mirror(grid, row=r)])
        total += sum([c for c in range(len(grid[0])) if is_mirror(grid, col=c)])
    return total


def part_two(inputs):
    total = 0
    grids = process_inputs(inputs)
    # breakpoint()
    for grid in grids:
        total += sum([r*100 for r in range(len(grid)) if is_mirror(grid, row=r, smudges=1, log=True)])
        print(total)
        total += sum([c for c in range(len(grid[0])) if is_mirror(grid, col=c, smudges=1, log=True)])
        print(total)
    return total


if __name__ == '__main__':
    # assert part_one(test) == 405
    print(part_two(test))
