import os
import sys

def get_inputs(day=1, split_lines=True):
    with open("inputs" + os.sep + f"{day}.txt") as file:
        contents = file.read()
    if split_lines:
        return contents.split("\n")
    return contents