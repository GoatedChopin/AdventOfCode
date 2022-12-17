from shapely.geometry import MultiPolygon, Polygon
from shapely.ops import unary_union

with open("inputs/15.txt") as file:
    inputs = file.readlines()
    inputs = [line.replace("\n", "").replace("Sensor at ", "") for line in inputs]
    inputs = [line.replace(": closest beacon is at ", "), ") for line in inputs]
    inputs = [line.replace("x=", "(").replace("y=", "") for line in inputs]
    inputs = [eval(line + ")") for line in inputs]


def manhattan_distance(sensor, beacon):
    dim = len(sensor)
    return sum(abs(sensor[i]-beacon[i]) for i in range(dim))


def abs_sum(delta):
    return sum(abs(i) for i in delta)


def move(position, delta):
    dim = len(position)
    return [position[i]+delta[i] for i in range(dim)]


def all_deltas(distance):
    checked_positions = set()
    for x in range(distance+1):
        for y in range(distance+1):
            if abs_sum((x, y)) <= distance:
                checked_positions.add((x, y))
                checked_positions.add((-x, y))
                checked_positions.add((x, -y))
                checked_positions.add((-x, -y))
            else:
                break
    return checked_positions


# def num_checked_positions(sensor, beacon):
#     checked_positions = 1
#     distance = manhattan_distance(sensor, beacon)
#     for i in range(1, distance+1):
#         checked_positions += 4*i
#     return checked_positions


def row_distance(sensor, rownum=2000000):
    return abs(sensor[1] - rownum)


def part_one(inputs):
    print("Getting distances")
    delta_map = {}
    for sensor, beacon in inputs:
        reach = manhattan_distance(sensor, beacon)
        distance = row_distance(sensor)
        if reach >= distance and reach not in delta_map:
            print("Getting all deltas for {}".format(reach))
            delta_map[reach] = all_deltas(reach)


    checked_positions = set()
    def all_checked_positions(sensor, deltas, y_filter=2000000):
        for x, y in deltas:
            next_position = move(sensor, (x, y))
            if next_position[1] == y_filter:
                checked_positions.add(next_position)

    return len(checked_positions)


# def find_square(sensor, beacon):
#     distance = manhattan_distance(sensor, beacon)
#     top = move(sensor, (0, distance))
#     right = move(sensor, (distance, 0))
#     down = move(sensor, (0, -distance))
#     left = move(sensor, (-distance, 0))
#     return (top, right, down, left)


# def find_dimension(squares, axis=0, func=max):
#     return func(func([s[axis] for s in square] for square in squares))


# def part_one(inputs):
#     points = [find_square(sensor, beacon) for sensor, beacon in inputs]
#     xmin, xmax = find_dimension(points, 0, min), find_dimension(points, 0, max)
#     ymin, ymax = find_dimension(points, 1, min), find_dimension(points, 1, max)
#     mapa = Polygon([(xmin, xmax), (xmin, ymax), (xmax, ymax), (xmax, ymin)])
#     print(mapa)
#     polygons = [Polygon(square) for square in points]
#     squares = MultiPolygon(polygons)
#     return unary_union(squares).intersection(mapa).area


# def part_one(inputs):
#     points = [find_square(sensor, beacon) for sensor, beacon in inputs]
#     polygons = [Polygon(square) for square in points]
#     diff_set = set()
#     for i1, p1 in enumerate(polygons):
#         for i2, p2 in enumerate(polygons):
#             if p1 != p2 and (i1, i2) not in diff_set and (i2, i1) not in diff_set:
#                 diff_set.add((i1, i2))
#                 if p1.intersects(p2):
#                     print("{} crosses {}".format(i1, i2))
#                     polygons[i1] = p1.difference(p2)
#     m = MultiPolygon(polygons)  # sum(p.area for p in polygons)
#     return unary_union(m).area


if __name__ == "__main__":
    print(part_one(inputs))