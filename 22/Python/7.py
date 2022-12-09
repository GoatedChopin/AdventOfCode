class directory:
    def __init__(self, name, parent=None) -> None:
        self.name = name
        self.children = {}
        self.parent = parent

    def get_size(self) -> int:
        size = 0
        for child in self.children.values():
            if isinstance(child, (directory, advent_file)):
                size += child.get_size()
        return size

    def add(self, child):
        self.children[child.name] = child

    def __eq__(self, __o: object) -> bool:
        return self.name == __o.name
    
    def __hash__(self) -> int:
        parent_string = ""
        current = self.parent
        while current:
            parent_string = parent_string + current.name
            current = current.parent
        return hash(self.name + parent_string)


class advent_file:
    def __init__(self, name, size) -> None:
        self.name = name
        self.size = size
    
    def __eq__(self, __o: object) -> bool:
        return self.name == __o.name and self.size == __o.size

    def __hash__(self) -> int:
        return hash(self.name)

    def get_size(self) -> int:
        return self.size


root = directory("root")
with open("inputs/7.txt") as file:
    inputs = [line.replace("\n", "") for line in file.readlines()]

current_dir = root
for i in inputs:

    if i.startswith("$"):  # it's a command, check for ls or cd
        output = i.replace("$ ", "").split(" ")
        if output[0] == "ls":  # no need to do anything yet, we'll worry about this if it's a problem.
            pass
        elif output[0] == "cd":
            if output[1] == "..":  # traverse up one directory
                current_dir = current_dir.parent
            elif output[1] == "/":  # go to the root directory
                current_dir = root
            else:
                current_dir = current_dir.children[output[1]]

    else:  # it's output from a command
        output = i.split(" ")
        print("Adding {} to {}".format(output[1], current_dir.name))
        if output[0] == "dir":  # sub-directory
            obj = directory(name=output[1], parent=current_dir)
        else:  # file
            obj = advent_file(name=output[1], size=int(output[0]))
        
        current_dir.add(obj)
        

def part_one_no_duplicates(root, visited=set()):
    if root in visited:
        return 0
    visited.add(root)
    total_size = 0
    if (current_size := root.get_size()) <= 100000:
        print("Subdirectory {} has size {}".format(root.name, current_size))
        return current_size
    else:
        for child in root.children.values():
            if isinstance(child, directory):
                subdir_total = part_one(child, visited)
                # print("Subdir {} has size {}".format(child.name, subdir_total))
                total_size += subdir_total
    return total_size


def part_one(root):
    total_size = 0
    if (current_size := root.get_size()) <= 100000:
        total_size += current_size
    for child in root.children.values():
        if isinstance(child, directory):
            subdir_total = part_one(child)
            total_size += subdir_total
    return total_size


smallest_acceptable_directory = directory("placeholder_directory")
smallest_acceptable_directory.add(advent_file("placeholder_file", float("inf")))
def part_two(root, current_free_memory):
    global smallest_acceptable_directory
    current_directory_size = root.get_size()
    if current_free_memory + current_directory_size >= 30000000 and current_directory_size < smallest_acceptable_directory.get_size():
        smallest_acceptable_directory = root
    for child in root.children.values():
        if isinstance(child, directory):
            part_two(child, current_free_memory)


if __name__ == "__main__":
    # print(part_one(root))
    current_free_memory = 70000000 - root.get_size()
    part_two(root, current_free_memory)
    print(smallest_acceptable_directory.get_size())

