import os
from llist import dllist, dllistnode


with open("inputs" + os.sep + "20.txt") as file:
    dll = dllist([(int(i), False) for i in file.readlines()])


def new_index(current_index, length_of_list, number):
    n_i = current_index + number
    if n_i > length_of_list:
        while n_i > length_of_list:
            n_i -= length_of_list
    elif n_i == 0:
        n_i = length_of_list
    elif n_i < 0:
        n_i += length_of_list * abs(n_i//length_of_list)
        n_i -= 1
    return n_i


def move(old_index, new_index):
    global dll
    old_node_num, _ = dll.nodeat(old_index).value
    new_node = dllistnode((old_node_num, True))
    if 0 < new_index+1 < len(dll):
        dll.insert(new_node, dll.nodeat(new_index+1))
    elif new_index == 0:
        dll.appendleft(new_node)
    else:
        dll.appendright(new_node)
    if new_index < old_index:
        old_index += 1
    del dll[old_index]


def part_one(dll, debug=False):
    dll_len = len(dll)
    numbers_moved = 0
    current_node, current_index = dll.first, 0
    while numbers_moved < dll_len:
        if debug:
            print([i[0] for i in dll])
        number, has_moved = current_node.value
        if not has_moved:
            n_index = new_index(current_index, dll_len, number)
            if debug:
                print("Moving {} at {} to {}".format(number, current_index, n_index))
            move(current_index, n_index)

            numbers_moved += 1

            if n_index > current_index:
                current_index -= 1
            elif n_index < current_index:
                current_index += 1

        current_index += 1
        if current_index == dll_len:
            current_index = 0
        current_node = dll.nodeat(current_index)


    # Find index of 0
    zero_index, current_node = 0, dll.first
    while current_node.value[0] != 0:
        current_node = current_node.next
        zero_index += 1
    print("0 is at {}".format(zero_index))


    # Find the indexes 1000, 2000, and 3000 after the zero_index
    checkpoints = (1000, 2000, 3000)
    i = 0
    total = 0
    while len(checkpoints) > 0:
        if i == checkpoints[0]:
            num, flag = current_node.value
            print("{} numbers after {} is {}".format(i, zero_index, num))
            total += num
            checkpoints = checkpoints[1:]

        if current_node.next:
            current_node = current_node.next
        else:
            current_node = dll.first
        i += 1
    print(total)

if __name__ == "__main__":
    # Real input
    part_one(dll, debug=False)
    print("\n")

    # Test input
    dll = dllist([(i, False) for i in [1, 2, -3, 3, -2, 0, 4]])
    part_one(dll, debug=True)
