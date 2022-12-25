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


def find_square(sensor, beacon):
    distance = manhattan_distance(sensor, beacon)
    top = move(sensor, (0, distance))
    right = move(sensor, (distance, 0))
    down = move(sensor, (0, -distance))
    left = move(sensor, (-distance, 0))
    return (top, right, down, left)


def find_dimension(squares, axis=0, func=max):
    return func(func([s[axis] for s in square] for square in squares))


def part_one(inputs):

    sensor_reach = {}
    def can_reach(sensor, point):
        return manhattan_distance(sensor, point) <= sensor_reach[sensor]

    points = []
    reach_sensors = []
    for sensor, beacon in inputs:
        sensor_reach[sensor] = manhattan_distance(sensor, beacon)
        if can_reach(sensor, (sensor[0], 2000000)):
            points.append(find_square(sensor, beacon))
            reach_sensors.append(sensor)

    xmin, xmax = find_dimension(points, 0, min), find_dimension(points, 0, max)

    reached_indexes = 0
    for x in range(xmin, xmax+1):
        for sensor in reach_sensors:
            if can_reach(sensor, (x, 2000000)):
                reached_indexes += 1
                break
    return reached_indexes


if __name__ == "__main__":
    print(part_one(inputs))