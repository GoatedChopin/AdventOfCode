from heapq import heappush, heappop


inputs = [[]]
with open("inputs/13.txt") as file:
    lines = [line.replace("\n", "") for line in file.readlines()]
    pair = 0
    for line in lines:
        if line == "":
            pair += 1
            inputs.append([])
        else:
            inputs[pair].append(eval(line))


def correct(pair):
    left, right = pair
    for l, r in zip(left, right):
        # print((l, r))
        if type(l) == int and type(r) == int:
            if l < r:
                return True
            elif l > r:
                return False
        
        elif type(l) == list and type(r) == list:
            if correct((l, r)) is True:
                return True
            elif correct((l, r)) is False:
                return False
        
        elif type(l) == list and type(r) == int:
            r = [r]
            if correct((l, r)) is True:
                return True
            elif correct((l, r)) is False:
                return False
        
        elif type(l) == int and type(r) == list:
            l = [l]
            if correct((l, r)) is True:
                return True
            elif correct((l, r)) is False:
                return False
    
    if len(left) < len(right):
        return True
    elif len(left) > len(right):
        return False

assert correct([[1,1,3,1,1],[1,1,5,1,1]])
assert correct([[[1],[2,3,4]], [[1],4]])
assert not correct([[9], [[8,7,6]]])
assert correct([[[4,4],4,4], [[4,4],4,4,4]])
assert not correct([[7,7,7,7], [7,7,7]])
assert correct([[], [3]])
assert not correct([[[[]]], [[]]])
assert not correct([[1,[2,[3,[4,[5,6,7]]]],8,9] ,[1,[2,[3,[4,[5,6,0]]]],8,9]])


def part_one(inputs):
    sum_correct_indexes = 0
    for i, pair in enumerate(inputs):
        if correct(pair):
            sum_correct_indexes += i + 1
    return sum_correct_indexes


def nested_size(array):
    size = 1
    for a in array:
        if type(a) == list:
            size += nested_size(a)
    return size


def first_element(line):
    if not line:  # empty list
        return 0
    
    first = line[0]
    if type(first) == int:
        return first
    
    return first_element(first)


def score(line):
    total_score = 0

    total_score += nested_size(line)  # how many lists are in the line?
    total_score += len(line)  # how many elements are in the line?
    total_score += 100 * first_element(line)

    return total_score


def part_two(inputs):
    key1, key2 = [[2]], [[6]]
    key1_index, key2_index = 1, 2
    for pair in inputs:
        left, right = pair
        if correct((left, key1)):
            key1_index += 1
        if correct((right, key1)):
            key1_index += 1
        if correct((left, key2)):
            key2_index += 1
        if correct((right, key2)):
            key2_index += 1

    decoder_key = key1_index * key2_index
    return decoder_key

if __name__ == "__main__":
    print(part_one(inputs))
    print(part_two(inputs))
