def part_2(file: str) -> int:
    with open(file, 'r') as inputs:
        lines = [parse_line(x) for x in inputs.readlines()]
        result = sum(lines)
        return result


def parse_line(line: str) -> int:
    lookup_table = {
            "one": 1,
            "two": 2,
            "three": 3,
            "four": 4,
            "five": 5,
            "six": 6,
            "seven": 7,
            "eight": 8,
            "nine": 9
            }
    first = None
    last = None
    for index, char in enumerate(line):
        if char.isnumeric():
            first = first or int(char)
            last = int(char)
        else:
            for string, number in lookup_table.items():
                if line[index:].startswith(string):
                    first = first or number
                    last = number

    return (first * 10) + last


if __name__ == "__main__":
    file = './inputs.txt'
    print(part_2(file))
