from common import standard_inputs


inputs = standard_inputs(7, True, True)
test = ["32T3K 765", "T55J5 684", "KK677 28", "KTJJT 220", "QQQJA 483"]

SCORE_KEY = {
    'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10, '9': 9, '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3, '2': 2
}


class CamelHand(object):
    def __init__(self, hand) -> None:
        self.hand = hand

    def __lt__(self, other):
        pass

    def __gt__(self, other):
        pass


def hand_score(hand):
    # print(hand)
    tab = {}
    for c in hand:
        if c not in tab:
            tab[c] = 0
        tab[c] += 1
    counts = {val: key for key, val in tab.items()}
    if 5 in counts:
        # print('Five of a kind')
        base = 50
    elif 4 in counts:
        # print('Four of a kind')
        base = 50
    elif 3 in counts and 2 in counts:
        # print('Full House')
        base = 40
    elif 3 in counts:
        # print('Three of a kind')
        base = 30
    elif 2 in counts:
        pairs = 0
        for key in tab.keys():
            if tab[key] == 2:
                pairs += 1
        if pairs == 1:
            # print('One pair')
            pass
        elif pairs == 2:
            # print('Two pair')
            pass
        base = 10 * pairs
    else:
        # print('High card')
        base = 1
    base *= (10**6)
    for ind, num in enumerate([SCORE_KEY[i] for i in hand]):
        base += (num*(10**(4-ind)))
    # extra = int('0'.join([str(SCORE_KEY[i]) for i in hand]))
    return base


def parse_inputs(inputs):
    hands = []
    bets = []
    for line in inputs:
        hand, bet = line.split(' ')
        bet = int(bet)
        hands.append(hand)
        bets.append(bet)
    return hands, bets


def part_one(inputs):
    hands, bets = parse_inputs(inputs)
    x = list(zip(hands, bets))
    x = sorted(x, key=lambda h: hand_score(h[0]))
    rank = 1
    total = 0
    for _, bet in x:
        score = (rank * bet)
        print(f"Adding {rank} * {bet} = {score} to {total}")
        total += score
        rank += 1
    return total


if __name__ == '__main__':
    print(part_one(inputs))
