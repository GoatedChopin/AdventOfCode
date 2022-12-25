import os


def get_inputs():
    with open("inputs" + os.sep + "25.txt") as file:
        inputs = file.readlines()
        inputs = [[c for c in line if c != "\n"] for line in inputs]
        return inputs


def get_snafu_bounds(places):
    upper_bound = "2"*places
    if places > 1:
        lower_bound = "1" + ("=" * (places - 1))
    else:
        lower_bound = "0"
    return snafu_to_decimal(lower_bound), snafu_to_decimal(upper_bound)


def in_snafu_bounds(decimal, lower_bound, upper_bound):
    return lower_bound <= decimal <= upper_bound


def snafu_to_decimal(snafu_string):
    power = len(snafu_string) - 1
    decimal = 0
    for c in snafu_string:
        match c:
            case '-': num = -1
            case '=': num = -2
            case other: num = int(c)
        decimal += (5 ** power) * num
        power -= 1
    return decimal


def get_snafu_places(decimal):
    i = 1
    found_bounds = False
    while not found_bounds:
        lower_bound, upper_bound = get_snafu_bounds(i)
        if in_snafu_bounds(decimal, lower_bound, upper_bound):
            found_bounds = True
        i += 1
    return i-1


def traverse_snafus(desired_decimal, decimal_places, previous_string=''):
    chars = ['2', '1', '0', '-', '=']
    if decimal_places == 1:
        for char in chars:
            print(previous_string + char)
            if snafu_to_decimal(previous_string + char) == desired_decimal:
                return previous_string + char
    for char in chars:
        possible = traverse_snafus(desired_decimal, decimal_places-1, previous_string + char)
        if possible is not None:
            return possible



def find_snafu(decimal):
    chars = ['2', '1', '0', '-', '=']
    places = get_snafu_places(decimal)
    snafu = '2' * places

    snafu_index = 0
    char_index = 0
    while snafu_to_decimal(snafu) != decimal:
        if char_index < 4:
            new_snafu = snafu[:snafu_index] + chars[char_index+1] + snafu[snafu_index+1:]
            if snafu_to_decimal(new_snafu) >= decimal:
                snafu = new_snafu
                # print(snafu)
                char_index += 1
            else:
                snafu_index += 1
                char_index = 0
        else:
            snafu_index += 1
            char_index = 0
    return snafu


def decimal_to_snafu(decimal):
    # num_places = get_snafu_places(decimal)
    # return traverse_snafus(decimal, num_places)
    return find_snafu(decimal)


def part_one(inputs):
    total = 0
    for snafu_string in inputs:
        total += snafu_to_decimal(snafu_string)
    return find_snafu(total)


if __name__ == "__main__":
    assert snafu_to_decimal("1=0") == 15
    assert snafu_to_decimal("1-0") == 20
    assert snafu_to_decimal("1=11-2") == 2022

    inputs = get_inputs()
    print(part_one(inputs))