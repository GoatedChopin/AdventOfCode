scores = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}
beats = {"X": "C", "Y": "A", "Z": "B", "A": "Z", "B": "X", "C": "Y"}
beaten_by = {val:key for key, val in beats.items()}

def load_plan():
    plan = []
    with open("inputs/2.txt", "r") as file:
        plan = [line.replace("\n", "").split(" ") for line in file.readlines()]
    return plan

def part_one(plan):
    score = 0
    for round in plan:
        print(round)
        score += scores[round[1]]
        print("\t\t{} bonus points for {}".format(scores[round[1]], round[1]))
        if round[0] == beats[round[1]]:
            score += 6
            print("\t{} beats {}".format(round[1], round[0]))
        elif scores[round[0]] == scores[round[1]]:
            score += 3
            print("\tTie")
        else:
            print("\tYou lose")
    return score


def sanity_test():
    assert part_one([('A', 'Y'), ('B', 'X'), ('C', 'Z')]) == 15


def part_two(plan):
    # X -> Lose, Y -> Draw, Z -> Win
    score = 0
    for round in plan:
        if round[1] == "X":  # We need to lose
            score += scores[beats[round[0]]]  # Add the score corresponding to whatever is beaten by the opponent's shape
        elif round[1] == "Y":  # We need to tie
            score += scores[round[0]]
            score += 3  # inherent bonus for tie-ing
        else:
            score += scores[beaten_by[round[0]]]
            score += 6
    return score


if __name__ == "__main__":
    plan = load_plan()
    sanity_test()
    print(part_one(plan))
    print(part_two(plan))