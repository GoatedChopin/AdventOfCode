from pprint import pprint

from common import standard_inputs


inputs = standard_inputs(5, True, True)

test = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""


class SeedMap(object):
    def __init__(self, source_entity: str, destination_entity: str, ranges: list) -> None:
        self.source = source_entity
        self.destination = destination_entity
        self.ranges = ranges

    def get_match(self, i: int):
        for destination_range, source_range, range_len in self.ranges:
            if source_range <= i < source_range + range_len:
                return destination_range + (i - source_range)
        return i

    def __hash__(self) -> int:
        return self.source.__hash__()


class Seed(object):
    def __init__(self, i: int, seed_maps: dict) -> None:
        self.seed = i
        self.soil = seed_maps['seed'].get_match(self.seed)
        self.fertilizer = seed_maps['soil'].get_match(self.soil)
        self.water = seed_maps['fertilizer'].get_match(self.fertilizer)
        self.light = seed_maps['water'].get_match(self.water)
        self.temperature = seed_maps['light'].get_match(self.light)
        self.humidity = seed_maps['temperature'].get_match(self.temperature)
        self.location = seed_maps['humidity'].get_match(self.humidity)


def parse_inputs(inputs):
    seed_maps = {}
    seeds = [int(i) for i in inputs[0].replace('seeds: ', '').strip().split(' ')]
    rules = []
    for line in inputs[1:]:
        if '-to-' in line:
            print(line)
            rules = []
            entities = line.strip().replace(' map:', '').split('-to-')
            source_entity, destination_entity = entities
        elif not line.strip():
            if rules:
                seed_maps[source_entity] = SeedMap(source_entity, destination_entity, rules)
        else:
            rules.append([int(i) for i in line.strip().split(' ')])
    if rules:
        seed_maps[source_entity] = SeedMap(source_entity, destination_entity, rules)
    return seeds, seed_maps


def unit_test():
    seeds, seed_maps = parse_inputs(test.split('\n'))
    seed_paths = []
    for i in seeds:
        seed_paths.append(Seed(i, seed_maps))
    assert min([seed.location for seed in seed_paths]) == 35


seeds, seed_maps = parse_inputs(inputs)


def part_one():
    seed_paths = []
    for i in seeds:
        seed_paths.append(Seed(i, seed_maps))
    return min([seed.location for seed in seed_paths])


def part_two():
    min_location = float('inf')
    for i in range(0, len(seeds), 2):
        start_range = seeds[i]
        length_range = seeds[i] + 1
        print(f"Searching seeds from {start_range} to {start_range + length_range}")
        for s in range(start_range, start_range+length_range):
            seed = Seed(s, seed_maps)
            if seed.location < min_location:
                print(f"New low: {seed.location}")
                min_location = seed.location
    return min_location


print(part_two())
