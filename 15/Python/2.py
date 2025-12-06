from common import get_inputs


inputs = get_inputs(2)
inputs = inputs.split('\n')
inputs = [(int(i) for i in line.split('x')) for line in inputs if line.strip()]


def paper_required(l: int, w: int, h: int) -> int:
    ordered = sorted((l, w, h))
    return (2*l*w) + (2*w*h) + (2*h*l) + (ordered[0] * ordered[1])


def ribbon_required(l: int, w: int, h: int) -> int:
    ordered = sorted((l, w, h))
    return (2*ordered[0]) + (2*ordered[1]) + (l*w*h)


def unit_test():
    assert paper_required(2, 3, 4) == 58, print(paper_required(2, 3, 4))
    assert paper_required(1, 1, 10) == 43, print(paper_required(1, 1, 10))
    assert ribbon_required(2, 3, 4) == 34, print(ribbon_required(2, 3, 4))
    assert ribbon_required(1, 1, 10) == 14, print(ribbon_required(1, 1, 10))


def part_one(inputs):
    print(sum([paper_required(*line) for line in inputs]))


def part_two(inputs):
    print(sum([ribbon_required(*line) for line in inputs]))


if __name__ == '__main__':
    unit_test()
    print(inputs)
    print(part_one(inputs))
    print(inputs)
    print(part_two(inputs))