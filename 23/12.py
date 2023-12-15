from heapq import heappush, heappop

from common import standard_inputs


inputs = standard_inputs(12, True, True)

test = [
    '???.### 1,1,3',
    '.??..??...?##. 1,1,3',
    '?#?#?#?#?#?#?#? 1,3,1,6',
    '????.#...#... 4,1,1',
    '????.######..#####. 1,6,5',
    '?###???????? 3,2,1'
]


def process_inputs(inputs):
    springs, orders = [], []
    for line in inputs:
        spring, order = line.split(' ')
        springs.append(spring.replace('..', '.'))
        orders.append([int(i) for i in order.split(',')])
    return springs, orders


def parse_order(spring):
    order = []
    run = 0
    for c in spring:
        if c == '#':
            run += 1
        elif c == '.':
            if run > 0:
                order.append(run)
                run = 0
        elif c == '?':
            order.append(-1)
    if run > 0:
        order.append(run)
        run = 0
    return order


def orders_match_greedy(spring, order):
    calc_order = []
    run = 0
    for c in spring:
        if c == '#':
            run += 1
        elif c == '.':
            if run > 0:
                if run != order[len(calc_order)]:
                    return False
                calc_order.append(run)
                run = 0
    if run > 0:
        if run != order[len(calc_order)]:
            return False
    return True


def parse_and_prune(spring, order):
    matches = 0
    permutations = [(0, [], spring, sum([c == '?' for c in spring]))]
    while permutations:
        print(f'{len(permutations)} in consideration:', end=' ')
        start, sub_order, current, qs = heappop(permutations)
        print(f'Considering {current} | Matches -> {matches} | {qs} ?\'s left | starting at {start}', end=' | ')
        if qs == 0:
            matches += parse_order(current) == order
        else:
            point = start
            run = 0
            while point <= len(current):
                if point == len(current):
                    break
                if current[point] == '#':
                    run += 1
                elif current[point] == '.':
                    if run > 0:
                        if len(sub_order) == len(order):
                            matches += parse_order(current) == order
                            break
                        elif len(sub_order) > len(order) or order[len(sub_order)] != run:
                            print(f"Non match found: {sub_order + [run]} vs {order}")
                            break
                        sub_order.append(run)
                        run = 0
                elif current[point] == '?':
                    dot = current[:point] + '.' + current[point+1:]
                    hashtag = current[:point] + '#' + current[point+1:]
                    # print(dot, ' | ', hashtag, run, point, sub_order, end=' ')
                    # if dot == '.#.#?#?#?#?#?#??':
                    #     breakpoint()
                    # dot_push, hashtag_push = False, False
                    # if run != 0 and run == order[len(sub_order)]:
                    #     sub_order.append(run)
                    #     dot_push = True
                    #     heappush(permutations, (point+1, sub_order, dot, qs-1))
                    # elif run == 0:
                    #     dot_push = True
                    #     heappush(permutations, (point+1, sub_order, dot, qs-1))
                    #     if len(sub_order) < len(order):
                    #         hashtag_push = True
                    #         while point > 0 and current[point] == '#':
                    #             point -= 1
                    #         heappush(permutations, (point, sub_order, hashtag, qs-1))
                    # elif run != 0 and run < order[len(sub_order)]:
                        # while point > 0 and current[point] == '#':
                        #     point -= 1
                        # hashtag_push = True
                        # heappush(permutations, (point, sub_order, hashtag, qs-1))
                    # # print(f'| Pushed {dot}: {dot_push} | Pushed {hashtag}: {hashtag_push}')
                    # print(run, point, sub_order, order, end=' | ')
                    # print(len(sub_order)-(current[start] == '#'), order, sub_order)
                    # if len(sub_order) >= len(order):
                    #     if sub_order == order:
                    #         matches += parse_order(current) == order
                    #     break
                    # else:
                    heappush(permutations, (0, [], dot, qs-1))
                    heappush(permutations, (0, [], hashtag, qs-1))
                    # elif run == order[len(sub_order)-(current[start] == '#')]:
                    #     print('Branch 1')
                    #     sub_order.append(run)
                    #     run = 0
                    #     heappush(permutations, (point+1, sub_order, dot, qs-1))
                    # elif run != 0 and run < order[len(sub_order)]:
                    #     while point > 0 and current[point] == '#':
                    #         point -= 1
                    #     print(f'Branch 2, {point}')
                    #     heappush(permutations, (point, sub_order, hashtag, qs-1))
                    # elif run > order[len(sub_order)]:
                    #     print('Branch 3')
                    #     break
                    # else:
                    #     print('Branch 4')
                    #     heappush(permutations, (point+1, sub_order, dot, qs-1))
                    #     heappush(permutations, (point+1, sub_order, hashtag, qs-1))
                    # print()
                    break
                point += 1
        print()
    return matches



# def parse_and_prune(spring, order, start=0, matches=0, questions=None):
#     if questions is None:
#         questions = sum([c == '?' for c in spring])
#     elif questions == 0:
#         is_match = parse_order(spring) == order
#         if is_match:
#             print(f'{spring} matches {order}')
#             return 1
#         return 0
#     for i in range(start, len(spring)+1):
#         if i == len(spring):
#             return (parse_order(spring) == order) + matches
#         elif spring[i] == '?':
#             if orders_match_greedy(spring[:i] + '#', order):
#                 matches += parse_and_prune(spring[:i] + '#' + spring[i+1:],
#                                            order=order,
#                                            start=i+1,
#                                            matches=matches,
#                                            questions=questions-1)
#             if orders_match_greedy(spring[:i] + '.', order):
#                 matches += parse_and_prune(spring[:i] + '.' + spring[i+1:],
#                                            order=order,
#                                            start=i+1,
#                                            matches=matches,
#                                            questions=questions-1)
#             break
#     return matches


# def part_one(inputs):
#     springs, orders = process_inputs(inputs)
#     configs = 0
#     for spring, order in zip(springs, orders):
#         permutations = [(0, spring)]
#         while permutations:
#             start, current = heappop(permutations)
#             if start == len(current):
#                 if parse_order(current) == order:
#                     configs += 1
#                 continue
#             for i in range(start, len(current)+1):
#                 if i < len(current) and current[i] == '?':
#                     dot = current[:i] + '.' + current[i+1:]
#                     hashtag = current[:i] + '#' + current[i+1:]
#                     heappush(permutations, (i+1, dot))
#                     heappush(permutations, (i+1, hashtag))
#                 elif i == len(current):
#                     heappush(permutations, (len(current), current))
#     return configs


def part_one(inputs):
    springs, orders = process_inputs(inputs)
    configs = []
    for spring, order in zip(springs, orders):
        # input(spring + ' ' + str(order))
        configs.append(parse_and_prune(spring, order))
    print(configs)
    return sum(configs)


if __name__ == '__main__':
    print(part_one(inputs))
