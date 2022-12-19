import os


class Blueprint:
    def __init__(self, blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost) -> None:
        self.id = blueprint_id
        self.ore_robot_cost = ore_robot_cost
        self.clay_robot_cost = clay_robot_cost
        self.obsidian_robot_cost = obsidian_robot_cost
        self.geode_robot_cost = geode_robot_cost
        
        self.ore_robots = 1
        self.clay_robots = 0
        self.obsidian_robots = 0
        self.geode_robots = 0

        self.resources = {"ore": 0, "clay": 0, "obsidian": 0}

        self.geodes = 0
        self.minute = 0

    def walk(self, minutes=24, debug=True):
        while self.minute < minutes:
            construct = self.manage_resources()
            self.step(choice = construct, debug=debug)
        return self.geodes

    def explore_paths(self, minutes=24):
        pass

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

    def manage_resources(self) -> None:
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

    def all_possible_choices(self):
        choices = []
        if self.can_afford(self.geode_robot_cost):
            choices.append("geode")
            return choices
        if self.can_afford(self.obsidian_robot_cost):
            choices.append("obsidian")
            return choices
        if self.can_afford(self.clay_robot_cost):
            choices.append("clay")
        if self.can_afford(self.ore_robot_cost):
            choices.append("ore")
        return choices

    def quality_level(self):
        return self.id * self.geodes

    def can_afford(self, cost_dict):
        for key, val in cost_dict.items():
            if self.resources[key] < val:
                return False
        return True


if __name__ == "__main__":
    blueprints = []
    with open("inputs" + os.sep + "19.test") as file:
        for line in file.readlines():
            blueprint_id = int(line.split(":", 1)[0].split(" ")[1])
            ore_robot_cost = {"ore": int(line.split(".", 1)[0].split(" ")[-2])}
            clay_robot_cost = {"ore": int(line.split(".", 2)[1].split(" ")[-2])}
            obsidian_robot_cost = {"ore": int(line.split(".", 3)[2].split(" ")[-5]), "clay": int(line.split(".", 3)[2].split(" ")[-2])}
            geode_robot_cost = {"ore": int(line.split(".", 4)[3].split(" ")[-5]), "obsidian": int(line.split(".", 4)[3].split(" ")[-2])}
            print(blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)
            blueprints.append(Blueprint(blueprint_id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost))

    total_quality_level = 0
    for blueprint in blueprints:
        print(blueprint.walk(), blueprint.quality_level())
        total_quality_level += blueprint.quality_level()
    print(total_quality_level)