import os


LEFT = (0, -1)
RIGHT = (0, 1)
UP = (-1, 0)
DOWN = (1, 0)
DISPLAY_CHARS = {
    LEFT: '<',
    RIGHT: '>',
    UP: '^',
    DOWN: 'v'
}


def standard_inputs(day=1, readlines=False, scrub_newlines=True):
    with open("inputs" + os.sep + str(day) + ".txt") as file:
        inputs = file.read() if not readlines else file.readlines()
    if scrub_newlines:
        inputs = [i.replace('\n', '') for i in inputs]
    return inputs


def move(coord, delta):
    r, c = coord
    rd, cd = delta
    return r + rd, c + cd


def in_bounds(coord, grid):
    r, c = coord
    return 0 <= r < len(grid) and 0 <= c < len(grid[0])


def lookup(coord, grid, default=None):
    r, c = coord
    if in_bounds(coord, grid):
        return grid[r][c]
    return default


def show_grid(grid, lines='\n', chars=''):
    grid = [[str(c) for c in line] for line in grid]
    s = lines.join([chars.join(line) for line in grid])
    return s