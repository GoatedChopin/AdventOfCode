from common import standard_inputs


inputs = standard_inputs(2, True)


def flatten(data):
    out = []
    for elem in data:
        if isinstance(elem, (list, tuple)):
            out += flatten(elem)
        else:
            out.append(elem)
    return out


def parse_game(line):
    game = {}
    game_id = int(line.split(': ')[0].split(' ')[-1])
    game['game_id'] = game_id
    dice = [p.strip().split(', ') for p in line.split(':')[-1].split('; ')]
    for pull in dice:
        pull = flatten([d.split(' ') for d in pull])
        for i in range(len(pull)):
            if i % 2 == 0:
                num = pull[i]
            else:
                color = pull[i]
                game[color] = max(int(num), game.get(color, 0))
    return game


def part_one(inputs):
    total = 5050
    limits = {'red': 12, 'green': 13, 'blue': 14}
    for line in inputs:
        game = parse_game(line)
        for key, val in limits.items():
            if game[key] > val:
                print(f'Skipping game {game["game_id"]}')
                total -= game['game_id']
                break
    return total


def part_two(inputs):

    def power_set(game):
        return game['red'] * game['blue'] * game['green']

    total = 0
    for line in inputs:
        game = parse_game(line)
        total += power_set(game)
    return total


print(inputs[0])
print(parse_game(inputs[0]))
print(part_one(inputs))
print(part_two(inputs))