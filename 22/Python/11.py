from heapq import heappush, heappop
from math import lcm

class Monkey:
    def __init__(self, items=[]) -> None:
        self.items = items
        self.operation = lambda x: x
        self.test = lambda x: x % 1 == 0
        self.outcome = {True: "Monkey 1", False: "Monkey 2"}
        self.business = 0
    
    def inspect_items(self, worry_func= lambda x: x // 3):
        changes = []
        for _ in range(len(self.items)):
            changes.append(self.__inspect__(worry_func))
        return changes

    def __inspect__(self, worry_func):
        self.business += 1
        item = self.items.pop()
        item = self.operation(item)
        item = worry_func(item)
        item_outcome = self.test(item)
        return self.outcome[item_outcome], item
    

def get_monkeys(filename="inputs/11.txt"):
    with open(filename) as file:
        lines = [line.strip().replace("\n", "") for line in file.readlines()]

    monkeys = []
    for line in lines:
        if line.startswith("Starting items: "):
            line = line.replace("Starting items: ", "")
            monkeys.append(Monkey([int(item) for item in line.split(", ")]))
        if line.startswith("Operation: "):
            line = line.replace("Operation: ", "")
            monkeys[-1].operation = eval("lambda " + line.replace("new =", "x:").replace("old", "x"))
        if line.startswith("Test: "):
            test_number = int(line.split(" ")[-1])
            monkeys[-1].test = eval("lambda x: x % {} == 0".format(test_number))
        if "true" in line:
            true_monkey = int(line.split(" ")[-1])
            monkeys[-1].outcome[True] = true_monkey
        if "false" in line:
            false_monkey = int(line.split(" ")[-1])
            monkeys[-1].outcome[False] = false_monkey
    
    return monkeys


def part_one(monkeys):
    for _ in range(20):
        for monkey in monkeys:
            changes = monkey.inspect_items()
            for destination_monkey, item in changes:
                monkeys[destination_monkey].items.append(item)

    monkey_heap = []
    for monkey in monkeys:
        heappush(monkey_heap, -monkey.business)

    total_business = 1
    for _ in range(2):
        total_business *= heappop(monkey_heap)

    return total_business


def sanity_check():
    test_monkeys = get_monkeys("inputs/11.test")
    assert part_one(test_monkeys) == 10605


def part_two(monkeys):
    for round in range(10000):
        for monkey in monkeys:
            changes = monkey.inspect_items(lambda x: x % 9699690)
            for destination_monkey, item in changes:
                monkeys[destination_monkey].items.append(item)
        if round % 500 == 0:
            print(round)

    monkey_heap = []
    for monkey in monkeys:
        heappush(monkey_heap, -monkey.business)

    total_business = 1
    for _ in range(2):
        total_business *= heappop(monkey_heap)

    return total_business

if __name__ == "__main__":
    monkeys = get_monkeys()
    sanity_check()
    print(part_one(monkeys))
    print(part_two(monkeys))