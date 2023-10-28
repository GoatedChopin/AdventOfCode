from common import get_inputs


inputs = get_inputs(5).split('\n')


FORBIDDEN = {'ab', 'cd', 'pq', 'xy'}
VOWELS = {'a', 'e', 'i', 'o', 'u'}


def nice_string(string):
    d = {}
    consecutive = False
    vowel_matches = 0
    for i, c in enumerate(string):
        if not consecutive and c in d and d[c] == i - 1:
            consecutive = True
        d[c] = i
        if c in VOWELS:
            vowel_matches += 1
        if string[i:i+2] in FORBIDDEN:
            return False
    return consecutive and vowel_matches > 2


def unit_test():
    assert nice_string('ugknbfddgicrmopn')
    assert nice_string('aaa')
    assert not nice_string('jchzalrnumimnmhp')
    assert not nice_string('haegwjzuvuyypxyu')
    assert not nice_string('dvszwmarrgswjxmb')


def part_one():
    print(len([string for string in inputs if nice_string(string)]))



if __name__ == '__main__':
    unit_test()
    part_one()