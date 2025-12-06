import os


def get_inputs(day=1):
    with open("inputs" + os.sep + str(day) + ".txt") as file:
        return file.read()
