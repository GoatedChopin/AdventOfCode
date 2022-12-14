
from collections import deque
from heapq import heappush, heappop

chr_map = {chr(i): i for i in range(97, 123)}
chr_map['E'] = 69  # Desired topmap position
chr_map['S'] = 83  # Current position

start, end = None, None

def char_to_num(c):
    return chr_map[c]

topmap = []
with open("inputs/12.txt") as file:
    topmap = file.readlines()
    for row in range(len(topmap)):
        topmap[row] = topmap[row].replace("\n", "")
        topmap[row] = [c for c in topmap[row]]
        for col in range(len(topmap[0])):
            c = topmap[row][col]
            topmap[row][col] = char_to_num(c)
            if c == 'E':
                end = row, col
                topmap[row][col] = 122
            elif c == 'S':
                start = row, col
                topmap[row][col] = 97
        

def valid_neighbors(row, col):
    global topmap

    def in_bounds(row, col):
        return 0 <= row < len(topmap) and 0 <= col < len(topmap[0])

    dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]  # Cardinal directions
    v_neighbors = []
    for rd, cd in dirs:
        new_row, new_col = row + rd, col + cd
        if in_bounds(new_row, new_col):  # Make sure the neighbor falls within the grid
            if topmap[new_row][new_col] <= topmap[row][col] + 1:  # Check for height no taller than 1 than current
                v_neighbors.append((new_row, new_col))

    return v_neighbors


visited=set()


def bfs(steps, row, col):
    global visited
    visited.add((row, col))
    neighbors = valid_neighbors(row, col)
    out = []
    for neighbor_row, neighbor_col in neighbors:
        if (neighbor_row, neighbor_col) not in visited:
            out.append((steps+1, neighbor_row, neighbor_col))
            visited.add((neighbor_row, neighbor_col))
    return out



def part_one(start):
    global visited
    queue = deque()
    queue.appendleft((0, start[0], start[1]))
    row, col = start
    while len(queue) > 0 and (row, col) != end:
        current = queue.pop()
        print(current)
        steps, row, col = current
        next_spots = bfs(steps, row, col)
        for spot in next_spots:
            queue.appendleft(spot)
    
    if (row, col) == end:
        return steps
    else:
        return -1

def part_two():
    global visited
    global topmap

    a_spots = []
    for row in range(len(topmap)):
        for col in range(len(topmap[0])):
            if topmap[row][col] == char_to_num('a'):
                a_spots.append((row, col))
    
    solutions = []
    for spot in a_spots:
        spot_path_length = part_one(spot)
        if spot_path_length != -1:
            heappush(solutions, spot_path_length)
        visited = set()
    
    return heappop(solutions)
    


if __name__ == "__main__":
    print(part_one(start))
    print(part_two())
