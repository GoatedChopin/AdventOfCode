test_string = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

test_outcome = 1651


def parse_line(line):
    line = line.replace("\n", "")\
        .replace("Valve ", "")\
        .replace("has flow rate=", "")\
        .replace("; tunnels lead to valves ", " [")\
        .replace("; tunnel leads to valve ", " [") + "]"
    # now is like "AA 0 (DD, II, BB)"
    line = line.split(" ", 2)
    # now is like ["AA", "0", "(DD, II, BB)"]
    line[1] = int(line[1])
    line[2] = eval(line[2].replace("[", "[\"").replace(",", "\",").replace(" ", " \"").replace("]", "\"]"))
    # now is like ["AA", 0, ("DD", "II", "BB")]
    return line


with open("inputs/16.txt") as file:
    inputs = file.readlines()
    inputs = [parse_line(line) for line in inputs]


valve_pressure_map = {}
valve_neighbor_map = {}
for i in inputs:
    valve, pressure, neighbors = i
    valve_pressure_map[valve] = pressure, False  # pressure from opening, is the valve open boolean flag
    valve_neighbor_map[valve] = neighbors


def calculate_pressure(pressure_map):
    pressure = 0
    keys = set()
    for key, value in pressure_map.items():
        if value[1] is True:
            keys.add(key)
            pressure += value[0]
    print("Valves {} are open:\n\tPressure increments by {}".format(keys, pressure))
    return pressure


def dfs(valve, pressure_released=0, minutes_left=30, neighbor_map={}, pressure_map={}, visited=set()):  # We'll try non-looping paths for now
    if minutes_left == 0:
        return pressure_released

    # print("Visiting {}".format(valve))
    visited.add(valve)
    valve_pressure = pressure_map[valve]
    pressure_released += calculate_pressure(pressure_map)
    
    if valve_pressure[0] != 0 and not valve_pressure[1]:  # Seems like we should be able to assume that a valve is worth opening as long as it releaves some pressure, there are only 15 non-zero valves
        valve_pressure_map[valve] = valve_pressure[0], True
        # print("Releasing valve {} for pressure of {}".format(valve, valve_pressure[0]))
        minutes_left -= 1
        pressure_released += calculate_pressure(pressure_map)
    
    for neighbor in valve_neighbor_map[valve]:
        if neighbor not in visited:
            pressure_released = max(pressure_released, dfs(neighbor, pressure_released, minutes_left-1, neighbor_map, pressure_map, visited))

    return pressure_released


def sanity_test():
    test_inputs = [parse_line(line) for line in test_string.split("\n")]



def part_one():
    return dfs("AA", neighbor_map=valve_neighbor_map, pressure_map=valve_pressure_map)


if __name__ == "__main__":
    print(valve_neighbor_map)
    # print(valve_pressure_map)
    # for valve, pressure in valve_pressure_map.items():
    #     if pressure[0] != 0:
    #         print(valve, pressure)
    print(part_one())
