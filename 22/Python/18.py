import os
from collections import deque

with open("inputs" + os.sep + "18.txt") as file:
    inputs = [tuple([int(i) for i in line.split(",")]) for line in file.readlines()]
    drop_map = {}
    for i in inputs:
        drop_map[i] = True

cardinal = [(1, 0, 0), (0, 1, 0), (0, 0, 1), (-1, 0, 0), (0, -1, 0), (0, 0, -1)]


def get_boundaries(inputs):
    xmin, xmax, ymin, ymax, zmin, zmax = [0] * 6
    for point in inputs:
        xmin = min(xmin, point[0])
        xmax = max(xmax, point[0])
        ymin = min(ymin, point[1])
        ymax = max(ymax, point[1])
        zmin = min(zmin, point[2])
        zmax = max(zmax, point[2])
    return xmin, xmax, ymin, ymax, zmin, zmax

def move(point, delta):
    return tuple(point[i]+delta[i] for i in range(len(point)))


def part_one(inputs):
    global cardinal
    coordinate_set = set()
    total_surface_area = 0
    for point in inputs:
        coordinate_set.add(point)
        surface_area = 6
        for delta in cardinal:
            if move(point, delta) in coordinate_set:
                surface_area -= 2
        total_surface_area += surface_area
    return total_surface_area


def touches_water(point, bounds):
    xmin, xmax, ymin, ymax, zmin, zmax = bounds
    return point[0] in (xmin-1, xmax+1) or point[1] in (ymin-1, ymax+1) or point[2] in (zmin-1, zmax+1)


def bfs(point, drop_map, bounds):
    global cardinal

    def get_neighbors(point):
        return (move(point, c) for c in cardinal) 
    
    visited = set()
    visited.add(point)
    queue = deque()
    queue.appendleft(point)
    while queue:
        current = queue.pop()

        if touches_water(current, bounds):
            return set()

        neighbors = get_neighbors(current)
        for neighbor in neighbors:
            if neighbor not in visited and neighbor not in drop_map:
                visited.add(neighbor)
                queue.appendleft(neighbor)

    return visited


def can_reach(point, target, drop_map):
    global cardinal

    def get_neighbors(point):
        return (move(point, c) for c in cardinal) 
    
    visited = set()
    visited.add(point)
    queue = deque()
    queue.appendleft(point)
    while queue:
        current = queue.pop()

        if current == target:
            return True

        neighbors = get_neighbors(current)
        for neighbor in neighbors:
            if neighbor not in visited and neighbor not in drop_map:
                visited.add(neighbor)
                queue.appendleft(neighbor)

    return False


def part_two(inputs, bounds):
    xmin, xmax, ymin, ymax, zmin, zmax = bounds
    naive_surface_area = part_one(inputs)
    for x in range(xmin, xmax+1):
        for y in range(ymin, ymax+1):
            for z in range(zmin, zmax+1):
                if (x, y, z) not in drop_map:
                    visited = bfs((x, y, z), drop_map, bounds)
                    for v in visited:
                        drop_map[v] = "Visited"
                    naive_surface_area -= part_one(visited)
    return naive_surface_area


def part_two_slow(inputs, bounds):
    xmin, xmax, ymin, ymax, zmin, zmax = bounds
    naive_surface_area = part_one(inputs)
    air_pockets = []
    for x in range(xmin, xmax+1):
        for y in range(ymin, ymax+1):
            for z in range(zmin, zmax+1):
                if (x, y, z) not in drop_map and not can_reach((x, y, z), (xmin-1, ymin-1, zmin-1), drop_map):
                    air_pockets.append((x, y, z))
    
    return naive_surface_area - part_one(air_pockets)


if __name__ == "__main__":
    print(part_one(inputs))
    bounds = get_boundaries(inputs)
    print(part_two(inputs, bounds))
    
