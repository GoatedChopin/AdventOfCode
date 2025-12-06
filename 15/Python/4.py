from hashlib import md5


inputs = 'ckczppom'


def part_one():
    for i in range(1000000):
        h = md5((inputs + str(i)).encode()).hexdigest()
        if h[:5] == '00000':
            print(h)
            return i


def part_two():
    for i in range(117946, 100000000000):
        h = md5((inputs + str(i)).encode()).hexdigest()
        if h[:6] == '000000':
            print(h)
            return i


if __name__ == '__main__':
    print(part_one())
    print(part_two())