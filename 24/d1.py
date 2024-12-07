from lib.inputs import get_inputs


inputs = get_inputs(1)

def part_one(inputs):
    total = 0
    l, r = [], []
    for line in inputs:
        line_items = [int(i) for i in line.split(" ") if i != '']
        if len(line_items) != 2:
            continue
        lv, rv = line_items
        l.append(lv)
        r.append(rv)
    for lv, rv in zip(sorted(l), sorted(r)):
        total += abs(lv - rv)
    return total

def part_two(inputs):
    l, r = [], []
    for line in inputs:
        line_items = [int(i) for i in line.split(" ") if i != '']
        if len(line_items) != 2:
            continue
        lv, rv = line_items
        l.append(lv)
        r.append(rv)
    def num_instances(i, l):
        return i * len([li for li in l if li == i])

    return sum([num_instances(i, r) for i in l])

if __name__ == '__main__':
    print(part_one(inputs))
    print(part_two(inputs))