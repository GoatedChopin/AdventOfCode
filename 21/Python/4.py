from collections import defaultdict

with open("inputs/4.txt") as file:
    inputs = [int(i) for i in file.readline().split(",")]
    boards = defaultdict(dict)
    num_boards = 0
    current_board = -1
    for line in file:
        line = line.strip()
        if not line or current_board == -1:
            print("")
            row = 0
            current_board += 1
            num_boards += 1
            for i in range(5):
                boards[current_board]["row_{}".format(i)] = set()
                boards[current_board]["col_{}".format(i)] = set()
                boards[current_board]["just_called"] = 0
        else:
            line = line.replace("  ", ",").replace(" ", ",")
            print(line)
            for col, val in enumerate([int(i) for i in line.replace("\n", "").split(",")]):
                boards[current_board]["row_{}".format(row)].add(val)
                boards[current_board]["col_{}".format(col)].add(val)
                if col == 4:
                    break
            row += 1


def part_one_score(board):
    score = 0
    for position in range(5):
        score += sum(board["row_{}".format(position)])
    score *= board["just_called"]
    return score

    
def part_one(inputs, boards):
    for i in inputs:
        for board in range(num_boards):
            for position in range(5):
                if i in boards[board]["row_{}".format(position)]:
                    print("{} in board {} row {}".format(i, board, position))
                    boards[board]["row_{}".format(position)].remove(i)
                    boards[board]["just_called"] = i
                    if len(boards[board]["row_{}".format(position)]) == 0:
                        return board
                if i in boards[board]["col_{}".format(position)]:
                    print("{} in board {} col {}".format(i, board, position))
                    boards[board]["col_{}".format(position)].remove(i)
                    boards[board]["just_called"] = i
                    if len(boards[board]["col_{}".format(position)]) == 0:
                        return board


def part_two(inputs, win_boards):
    win_boards = []
    for i in inputs:
        for board in range(num_boards):
            if board not in win_boards:
                for position in range(5):
                    if i in boards[board]["row_{}".format(position)]:
                        print("{} in board {} row {}".format(i, board, position))
                        boards[board]["row_{}".format(position)].remove(i)
                        boards[board]["just_called"] = i
                        if len(boards[board]["row_{}".format(position)]) == 0:
                            if board not in win_boards:
                                win_boards.append(board)
                    if i in boards[board]["col_{}".format(position)]:
                        print("{} in board {} col {}".format(i, board, position))
                        boards[board]["col_{}".format(position)].remove(i)
                        boards[board]["just_called"] = i
                        if len(boards[board]["col_{}".format(position)]) == 0:
                            if board not in win_boards:
                                win_boards.append(board)
    return win_boards[-1]


if __name__ == "__main__":
    # winning_board = part_one(inputs, boards)
    # print(part_one_score(boards[winning_board]))
    loser_board = part_two(inputs, boards)
    print(part_one_score(boards[loser_board]))