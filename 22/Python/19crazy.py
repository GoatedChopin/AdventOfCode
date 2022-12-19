import os
from collections import defaultdict


class Blueprint:
    def __init__(self, blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost) -> None:
        self.id = blueprint_id

        self.cost = {}
        self.cost["ore"] = ore_robot_cost
        self.cost["clay"] = clay_robot_cost
        self.cost["obsidian"] = obsidian_robot_cost
        self.cost["geodes"] = geode_robot_cost

        self.ore_robot_cost = ore_robot_cost
        self.clay_robot_cost = clay_robot_cost
        self.obsidian_robot_cost = obsidian_robot_cost
        self.geode_robot_cost = geode_robot_cost
        
        self.ore_robots = 1
        self.clay_robots = 0
        self.obsidian_robots = 0
        self.geode_robots = 0

        self.resources = {"ore": 0, "clay": 0, "obsidian": 0, "geodes": 0}
        self.minute = 0

    def walk(self, minutes=24, debug=True):
        while self.minute < minutes:
            construct = self.best_choice()
            self.step(choice = construct, debug=debug)
        return self.resources["geodes"]

    def step(self, choice, bound=25, debug=False) -> int:
        self.minute += 1

        self.resources["ore"] += self.ore_robots
        self.resources["clay"] += self.clay_robots
        self.resources["obsidian"] += self.obsidian_robots
        self.geodes += self.geode_robots

        match choice:
            case "geode": 
                self.geode_robots += 1
                self.use_resources(self.geode_robot_cost)
            case "obsidian": 
                self.obsidian_robots += 1
                self.use_resources(self.obsidian_robot_cost)
            case "clay": 
                self.clay_robots += 1
                self.use_resources(self.clay_robot_cost)
            case "ore": 
                self.ore_robots += 1
                self.use_resources(self.ore_robot_cost)

        if debug:
            print("\t", self.minute, self.resources, choice)

    def use_resources(self, cost_dict):
        for key, val in cost_dict.items():
            self.resources[key] -= val

    def set_resources(self, resource_dict):
        self.resources = resource_dict

    def explore_path(self, robots, choice):
        resources = defaultdict(int)
        resources["ore"] = robots["ore"]
        resources["clay"] = robots["clay"]
        resources["obsidian"] = robots["obsidian"]
        resources["geodes"] = robots["geodes"]

        match choice:
            case "geode": 
                robots["geodes"] += 1
                cost_dict = self.geode_robot_cost
            case "obsidian": 
                robots["obsidian"] += 1
                cost_dict = self.obsidian_robot_cost
            case "clay": 
                robots["clay"] += 1
                cost_dict = self.clay_robot_cost
            case "ore": 
                robots["ore"] += 1
                cost_dict = self.ore_robot_cost
            case None:
                cost_dict = defaultdict(int)
        return resources, robots, cost_dict

    def best_choice(self) -> None:
        # 5 cases -> we can construct a robot or do nothing, with the more expensive robots being given priority over every robot less expensive than itself.
        if self.can_afford(self.geode_robot_cost):
            return "geode"
        elif self.can_afford(self.obsidian_robot_cost):
            return "obsidian"
        elif self.can_afford(self.clay_robot_cost):
            return "clay"
        elif self.can_afford(self.ore_robot_cost):
            return "ore"
        else:
            return None

    def quality_level(self):
        return self.id * self.resources["geodes"]

    def can_afford(self, cost_dict):
        for key, val in cost_dict.items():
            if self.resources[key] < val:
                return False
        return True


blueprints = []
with open("inputs" + os.sep + "19.test") as file:
    for line in file.readlines():
        blueprint_id = int(line.split(":", 1)[0].split(" ")[1])
        ore_robot_cost = {"ore": int(line.split(".", 1)[0].split(" ")[-2]), "clay": 0, "obsidian": 0}
        clay_robot_cost = {"ore": int(line.split(".", 2)[1].split(" ")[-2]), "clay": 0, "obsidian": 0}
        obsidian_robot_cost = {"ore": int(line.split(".", 3)[2].split(" ")[-5]), "clay": int(line.split(".", 3)[2].split(" ")[-2]), "obsidian": 0}
        geode_robot_cost = {"ore": int(line.split(".", 4)[3].split(" ")[-5]), "clay": 0, "obsidian": int(line.split(".", 4)[3].split(" ")[-2])}
        print(blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)
        blueprints.append(Blueprint(blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost))


def can_afford(ore, clay, obsidian, cost_dict):
    return ore >= cost_dict["ore"] and clay >= cost_dict["clay"] and obsidian >= cost_dict["obsidian"]


def all_possible_choices(blueprint, ore, clay, obsidian):
    choices = []
    if can_afford(ore, clay, obsidian, blueprint.cost["geodes"]):
        choices.append("geodes")
        return choices  # always the best choice
    if can_afford(ore, clay, obsidian, blueprint.cost["obsidian"]):
        choices.append("obsidian")
        return choices  # might always be the best choice, when geode is not possible
    if can_afford(ore, clay, obsidian, blueprint.cost["clay"]):
        choices.append("clay")
    if can_afford(ore, clay, obsidian, blueprint.cost["ore"]):
        choices.append("ore")
    return choices + ["Do nothing"]


def build_robot(blueprint, ore, clay, obsidian, choice):
    return ore - blueprint.cost[choice]["ore"], clay - blueprint.cost[choice]["clay"], obsidian - blueprint.cost[choice]["obsidian"]

checkpoint_memo = {}
minute_memo = {}
def dfs(blueprint_index, ore=0, clay=0, obsidian=0, geodes=0, r_ore=1, r_clay=0, r_obsidian=0, r_geodes=0, minute=0, bound=24):
    global blueprints
    global checkpoint_memo
    blueprint = blueprints[blueprint_index]
    if minute == bound:
        return geodes
    elif (blueprint_index, ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes, minute) in checkpoint_memo:
        return checkpoint_memo[(blueprint_index, ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes, minute)]
    elif (blueprint_index, minute) in minute_memo:
        previous_best = minute_memo[(blueprint_index, minute)]
        prune_path = True
        for current, prev in zip([ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes], previous_best):
            if current > prev:
                prune_path = False
        if prune_path:
            return 0

    choices = all_possible_choices(blueprint, ore, clay, obsidian)
    ore += r_ore
    clay += r_clay
    obsidian += r_obsidian
    geodes += r_geodes
    print("{} -> {}: {}, {}, {}; {}, {}, {}".format(minute, choices, ore, clay, obsidian, r_ore, r_clay, r_obsidian))
    for choice in choices:
        # print("\t\t taking choice {} at minute {}".format(choice, minute))
        if choice != "Do nothing":
            path_ore = ore - blueprint.cost[choice]["ore"]
            path_clay = clay - blueprint.cost[choice]["clay"]
            path_obsidian = obsidian - blueprint.cost[choice]["obsidian"]
            path_r_ore, path_r_clay, path_r_obsidian, path_r_geodes = r_ore, r_clay, r_obsidian, r_geodes
            match choice:
                case "ore": path_r_ore += 1
                case "clay": path_r_clay += 1
                case "obsidian": path_r_obsidian += 1
                case "geodes": path_r_geodes += 1
            path_geodes = dfs(blueprint_index, path_ore, path_clay, path_obsidian, geodes, path_r_ore, path_r_clay, path_r_obsidian, path_r_geodes, minute+1)
        else:
            path_geodes = dfs(blueprint_index, ore, clay, obsidian, geodes, r_ore, r_clay, r_obsidian, r_geodes, minute+1)
        geodes = max(geodes, path_geodes)

    if (blueprint_index, minute) not in minute_memo:
        previous_best = (0, 0, 0, 0, 0, 0, 0)
        minute_memo[(blueprint_index, minute)] = previous_best
    else:
        previous_best = minute_memo[(blueprint_index, minute)]
    best_path = True
    for current, prev in zip([ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes], previous_best):
        if current < prev:
            best_path = False
    if best_path:
        minute_memo[(blueprint_index, minute)] = (ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes)
    checkpoint_memo[(blueprint_index, ore, clay, obsidian, r_ore, r_clay, r_obsidian, r_geodes, minute)] = geodes
    return geodes


if __name__ == "__main__":
    total_quality_level = 0
    for i, blueprint in enumerate(blueprints[:1]):
        print("Optimizing blueprint #{}".format(blueprint.id))
        geodes = dfs(i)
        print("\t{} geodes".format(geodes))
        total_quality_level += geodes
    print(total_quality_level)