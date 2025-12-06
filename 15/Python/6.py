from common import get_inputs


def process_input(line: list[str]):
    if line[0] == 'toggle':
        sr, sc = (int(i) for i in line[1].split(','))
        er, ec = (int(i) for i in line[3].split(','))
        return sr, sc, er, ec, True, False
    elif line[1] == 'on':
        sr, sc = (int(i) for i in line[2].split(','))
        er, ec = (int(i) for i in line[4].split(','))
        return sr, sc, er, ec, False, True
    else:
        sr, sc = (int(i) for i in line[2].split(','))
        er, ec = (int(i) for i in line[4].split(','))
        return sr, sc, er, ec, False, False


inputs = get_inputs(6)
inputs = inputs.split('\n')
inputs = [line.split(' ') for line in inputs]
inputs = [process_input(line) for line in inputs if len(line) > 1]
new_grid = lambda x: [[0]*1000]*1000
GRID = new_grid(None)


def show_grid():
    global GRID
    for row in GRID:
        print(''.join((str(i) for i in row)))


def total_lights():
    global GRID
    return sum(sum(row) for row in GRID)


def take_instruction(sr: int, sc: int, er: int, ec: int, toggle: bool=False, turn_on: bool=True):
    global GRID
    for r in range(sr, er+1):
        for c in range(sc, ec+1):
            if toggle:
                GRID[r][c] = 0 if GRID[r][c] == 1 else 1
            else:
                GRID[r][c] = 1 if turn_on else 0


def unit_test():
    global GRID
    take_instruction(0, 0, 2, 2, False, True)
    assert total_lights() == 9, total_lights()
    GRID = new_grid(None)


def part_one():
    for instruction in inputs:
        take_instruction(*instruction)
    show_grid(GRID)
    return total_lights()


if __name__ == '__main__':
    unit_test()
    print(part_one())