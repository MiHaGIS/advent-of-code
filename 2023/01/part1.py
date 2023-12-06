def part_1(file: str) -> int:
    with open(file, 'r') as inputs:
        calibration_values = [line_parse(x) for x in inputs.readlines()]
        result = sum(calibration_values)
        return result


def line_parse(line: str) -> int:
    first = None
    last = None
    for char in line:
        if char.isnumeric():
            first = first or int(char)
            last = int(char)
    result = (first * 10) + last
    return result


if __name__ == "__main__":
    result = part_1('./inputs.txt')
    print(result)
