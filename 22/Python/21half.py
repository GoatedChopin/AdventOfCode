import os
from collections import defaultdict


with open("inputs" + os.sep + "21.txt") as file:
    inputs = [line.replace("\n", "") for line in file.readlines()]

values = {}  # simple key value store for evaluated expressions
dependencies = defaultdict(list)  # directed acyclic graph of dependencies. a: b means b depends on a's evaluation
depends_on = defaultdict(list)  # dag representing which keys a key depends on. a: b means a depends on b's evaluation
for i in inputs:
    key, value = i.split(": ")
    if len(value.split(" ")) == 1:
        values[key] = int(value)
    else:
        values[key] = value.split(" ")
        val1, operator, val2 = values[key]
        dependencies[val1].append(key)  # val1 is needed to evaluate key
        dependencies[val2].append(key)  # val2 is needed to evaluate key
        depends_on[key].extend([val1, val2])


def evaluate(key):
    global values
    if type(values[key]) == int:
        return values[key]
    else:
        val1, operator, val2 = values[key]
        return int(eval(" ".join([str(evaluate(val1)), operator, str(evaluate(val2))])))


def part_one():
    print(evaluate("root"))


def x_depends_on_y(x, y) -> bool:
    global depends_on
    x_ancestors = depends_on[x]
    if y in depends_on[x]:
        return True
    for ancestor in x_ancestors:
        if x_depends_on_y(ancestor, y):
            return True
    return False


def opposite_operator(operator):
    match operator:
        case "+": return "-"
        case "-": return "+"
        case "*": return "/"
        case "/": return "*"


def inverse(x):
    return x**-1


def unwrap_children(our_key="humn", fixed_value_key="fcjl", variable_value_key="brrs"):
    global values
    global depends_on

    for child_key in depends_on[variable_value_key]:
        if type(values[child_key]) == int:
            pass
        else:
            val1, operator, val2 = values[child_key]
            left_depends = x_depends_on_y(val1, our_key)
            right_depends = x_depends_on_y(val2, our_key)

            if left_depends and not right_depends:
                values[fixed_value_key] = eval(" ".join([str(values[fixed_value_key]), opposite_operator(operator), str(evaluate(val2))]))
                unwrap_children(variable_value_key=val1)
            elif not left_depends and right_depends:
                if operator == "/":
                    print("Time to invert, this is my nightmare")
                    values[fixed_value_key] = inverse(values[fixed_value_key])
                    values[fixed_value_key] *= val1
                else:
                    values[fixed_value_key] = eval(" ".join([str(values[fixed_value_key]), opposite_operator(operator), str(evaluate(val1))]))
                unwrap_children(variable_value_key=val2)
            elif not left_depends and not right_depends:
                print("This whole branch is useless: {} -> {} and {}".format(child_key, val1, val2))
            else:
                print("Both sides depend on humn: {} -> {} and {}".format(child_key, val1, val2))


def part_two():
    not_humn, humn = 0, 0
    for key in values.keys():
        if not x_depends_on_y(key, "humn") and key != "root":  # independent of our "shout" number
            not_humn += 1
            values[key] = int(evaluate(key))
        else:
            humn += 1
    print(not_humn, humn)
    unwrap_children()

    val1, operator, val2 = values["root"]
    if x_depends_on_y(val2, "humn"):
        fixed_val = val1
        variable_val = val2
    else:
        fixed_val = val2
        variable_val = val1
    
    print(fixed_val, variable_val)
    breakpoint()


# def part_two():
#     # for key in values.keys():
#     #     if not x_depends_on_y(key, "humn") and key != "root":  # independent of our "shout" number
#     #         values[key] = int(evaluate(key))
    
#     val1, operator, val2 = values["root"]
#     if x_depends_on_y(val1, "humn"):
#         fixed_val = val1
#         variable_val = val2
#     else:
#         fixed_val = val2
#         variable_val = val1
    
#     values["humn"] = 3831110828032

#     fixed_evaluation = evaluate(fixed_val)
#     variable_evaluation = evaluate(variable_val)
#     while fixed_evaluation != variable_evaluation:
#         if variable_evaluation < fixed_evaluation:
#             values["humn"] += int(fixed_evaluation / variable_evaluation)
#         elif variable_evaluation > fixed_evaluation:
#             values["humn"] -= int(variable_evaluation / fixed_evaluation)
#         variable_evaluation = evaluate(variable_val)
#         print("humn {} gives variable value {} against fixed value {} with absolute delta {}".format(values["humn"], variable_evaluation, fixed_evaluation, abs(variable_evaluation-fixed_evaluation)))
#     return values["humn"]




if __name__ == "__main__":
    # part_one()
    part_two()