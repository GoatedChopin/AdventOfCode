from collections import defaultdict

from common import standard_inputs


inputs = standard_inputs(4, True)


test = ['Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53',
        'Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19',
        'Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14 1',
        'Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83',
        'Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36',
        'Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11']


def process_card(line):
    card = line.replace('  ', ' ').split(': ')[-1]
    winning = set(card.split(' | ')[0].split(' '))
    ours = card.split(' | ')[-1].split(' ')
    points = 0
    for o in ours:
        if o in winning:
            points = points * 2 if points else 1
    return points


def num_copies(line):
    card = line.replace('  ', ' ').split(': ')[-1]
    winning = set(card.split(' | ')[0].split(' '))
    ours = card.split(' | ')[-1].split(' ')
    copies = 0
    for o in ours:
        if o in winning:
            copies += 1
    return copies


def part_one(inputs):
    total = 0
    for line in inputs:
        total += process_card(line)
    return total


def part_two(inputs):
    total_cards = 0
    copies = {}
    for i, line in enumerate(inputs):
        n = num_copies(line)
        card_number = i + 1
        if card_number not in copies:
            copies[card_number] = 1
        for _ in range(copies.get(card_number, 1)):
            for card in range(card_number+1, card_number+1+n):
                if card not in copies:
                    copies[card] = 1
                copies[card] += 1
                total_cards += 1
    return total_cards + len(inputs)


if __name__ == '__main__':
    print(part_one(inputs))
    print(part_two(inputs))
