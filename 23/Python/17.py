import copy
from heapq import heappush, heappop

from common import standard_inputs, lookup, in_bounds, move, show_grid, LEFT, UP, RIGHT, DOWN, DISPLAY_CHARS


inputs = standard_inputs(17, True, True)
test = [
    '2413432311323',
    '3215453535623',
    '3255245654254',
    '3446585845452',
    '4546657867536',
    '1438598798454',
    '4457876987766',
    '3637877979653',
    '4654967986887',
    '4564679986453',
    '1224686865563',
    '2546548887735',
    '4322674655533'
]

OPPOSITE_DELTAS = {
    LEFT: RIGHT,
    RIGHT: LEFT,
    UP: DOWN,
    DOWN: UP
}



def process_inputs(inputs):
    return [[int(c) for c in line] for line in inputs]


def part_one(inputs, start=(0, 0), display_grid=True):
    grid = process_inputs(inputs)
    display_grid = copy.deepcopy(grid)

    end = (len(grid)-1, len(grid[0])-1)
    best_heat_loss = float('inf')

    forward, heat_loss = 1, lookup(start, grid)

    visited = [set([start]), set([start])]
    heap = [(heat_loss, forward, RIGHT, start, 0), (heat_loss, forward, DOWN, start, 1)]
    while heap:
        print(heap)
        heat_loss, forward, prev_delta, position, visited_index = heappop(heap)
        if display_grid:
            r, c = position
            display_grid[r][c] = DISPLAY_CHARS[prev_delta]
            print(show_grid(display_grid))
            print(position, prev_delta, forward, heat_loss)
            # breakpoint()

        if position == end:
            visited[visited_index] = 0
            best_heat_loss = min(heat_loss, best_heat_loss)
            breakpoint()
            continue
        elif heat_loss > best_heat_loss:
            continue

        for delta in [RIGHT, DOWN, LEFT, UP]:
            next_position = move(position, delta)
            if not isinstance(visited[visited_index], int) and next_position not in visited[visited_index]:
                if not delta == OPPOSITE_DELTAS[prev_delta]:
                    if in_bounds(next_position, grid):
                        if not (delta == prev_delta and forward == 3):
                            if delta == prev_delta:
                                forward += 1
                            else:
                                forward = 1
                            next_visited_index = len(visited)
                            visited.append(copy.deepcopy(visited[visited_index]).union({next_position}))
                            visited[visited_index] = 0
                            heappush(heap, (heat_loss+lookup(next_position, grid), forward, delta, next_position, next_visited_index))
                        else:
                            print(f"Must turn! Was going {prev_delta} and forward = {forward}")
                    else:
                        print(f"Coord is not in bounds: {next_position}")
    return best_heat_loss


# def benchmark_grid(grid, start_delta=RIGHT):
#     start = (0, 0)
#     rows, cols = len(grid), len(grid[0])
#     end = (rows-1, cols-1)
#     heat_loss = 0
#     forward = 1
#     delta = start_delta
#     go_down = False
#     while start != end:
#         heat_loss += lookup(start, grid)
#         start = move(start, delta)
#         forward += 1
#         at_edge = start[1] == end[1]
#         must_turn = forward == 3
#         if go_down:
#             delta = DOWN
#             forward = 1
#             go_down = False
#         elif at_edge and not (delta == DOWN and must_turn):
#             if delta == DOWN:
#                 forward += 1
#             delta = DOWN
#         elif at_edge and (delta == DOWN and must_turn):
#             go_down = True
#             delta = LEFT
#         elif must_turn:
#             delta = DOWN if delta == RIGHT else RIGHT
#             forward = 1
#     heat_loss += lookup(end, grid)
#     return heat_loss


# def part_one(inputs):
#     grid = process_inputs(inputs)
#     end = (len(grid)-1, len(grid[0])-1)

#     current_best = benchmark_grid(grid)
#     breakpoint()

#     def dfs(start=(0, 0), heat_loss=0, prev_delta=None, forward=1, visited=set([(0, 0)])):
#         if start == end:
#             return heat_loss
#         elif heat_loss >= current_best:
#             return heat_loss
        
#         print(f"Visiting {start}")
#         paths = []
#         for delta in [UP, DOWN, LEFT, RIGHT]:
#             next_position = move(start, delta)
#             if next_position not in visited:
#                 if not delta == OPPOSITE_DELTAS[prev_delta] and in_bounds(next_position, grid) and not (delta == prev_delta and forward == 3):
#                     if delta == prev_delta:
#                         forward += 1
#                     else:
#                         forward = 1
#                     paths.append(dfs(next_position, heat_loss + lookup(next_position, grid), delta, forward, visited.union({next_position})))
#         if paths:
#             # print(paths)
#             return min(paths)
#         else:
#             return float('inf')
    
#     return min(dfs(prev_delta=RIGHT, heat_loss=lookup((0, 0), grid)), dfs(prev_delta=DOWN, heat_loss=lookup((0, 0), grid)))



def part_two(inputs):
    pass



if __name__ == '__main__':
    print(part_one(test))
    breakpoint()
    print(part_one(inputs))
    assert part_two(test) == None
    print(part_two(inputs))