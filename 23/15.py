from heapq import heappush

from common import standard_inputs


inputs = standard_inputs(15, False, False)


def process_inputs(inputs):
    return [c for c in inputs.strip().replace('\n', '').split(',')]


def hash(s):
    subtotal = 0
    for c in s:
        subtotal += ord(c)
        subtotal *= 17
        subtotal = subtotal % 256
    return subtotal



def part_one(inputs):
    instructions = process_inputs(inputs)
    total = 0
    for s in instructions:
        total += hash(s)
    return total


def print_mirrors(mirrors):
    for i in range(256):
        if i in mirrors and len(mirrors[i][0]):
            print(f'\t{i} | {mirrors[i][1]}')


def part_two(inputs):
    instructions = process_inputs(inputs)
    mirrors = {}
    for s in instructions:
        mirrors[hash(s.replace('-', '').replace('=', ''))] = set(), []
    
    for s in instructions:
        print(s)
        if s[-1] == '-':
            label = s.replace('-', '')
            inventory, mirror_order = mirrors[hash(label)]
            if label in inventory:
                inventory.remove(label)
                mirror_order = [(l, f) for l, f in mirror_order if l != label]
            mirrors[hash(label)] = inventory, mirror_order
        elif '=' in s:
            label, focal = s.split('=')
            focal = int(focal)
            inventory, mirror_order = mirrors[hash(label)]
            if label in inventory:
                for i, (l, f) in enumerate(mirror_order):
                    if l == label:
                        mirror_order[i] = (label, focal)
                mirrors[hash(label)] = inventory, mirror_order
            else:
                inventory.add(label)
                mirrors[hash(label)] = inventory, mirror_order + [(label, focal)]
        # print_mirrors(mirrors)
    

    def value(box, position, focal):
        subtotal = box + 1
        subtotal *= position
        subtotal *= focal
        return subtotal

    total = 0
    for i in range(256):
        if i in mirrors:
            inventory, mirror_order = mirrors[i]
            position = 1
            for label, focal in mirror_order:
                total += value(i, position, focal)
                # print(label, value(i, position, focal))
                position += 1
    return total



if __name__ == '__main__':
    assert part_one('HASH') == 52
    print(part_one(inputs))
    print(part_two('rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'))
    breakpoint()
    print(part_two(inputs))