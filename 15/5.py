from common import get_inputs


inputs = get_inputs(5).split('\n')


FORBIDDEN = {'ab', 'cd', 'pq', 'xy'}
VOWELS = {'a', 'e', 'i', 'o', 'u'}


def nice_string_one(string):
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


def nice_string_two(string):
    d = {}
    pairs = False
    letter_gap = False
    for i, c in enumerate(string):
        if i < len(string) - 1:
            pair = c + string[i+1]
            if pair in d and d[pair] < i - 1:
                pairs = True
                print(f"Pair: {pair}")
            d[pair] = i
        if i < len(string) - 2:
            if string[i+2] == c:
                letter_gap = True
                print(f'Gap: {string[i:i+3]}')
    return pairs and letter_gap


def unit_test():
    assert nice_string_one('ugknbfddgicrmopn')
    assert nice_string_one('aaa')
    assert not nice_string_one('jchzalrnumimnmhp')
    assert not nice_string_one('haegwjzuvuyypxyu')
    assert not nice_string_one('dvszwmarrgswjxmb')
    assert nice_string_two('qjhvhtzxzqqjkmpb')
    assert nice_string_two('xxyxx')
    assert not nice_string_two('uurcxstgmygtbstg')
    assert not nice_string_two('ieodomkazucvgmuy')


def part_one():
    print(len([string for string in inputs if nice_string_one(string)]))


def part_two():
    print(len([string for string in inputs if nice_string_two(string)]))


if __name__ == '__main__':
    unit_test()
    part_one()
    part_two()