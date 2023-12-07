from part2 import part_2, parse_line

def test_part_2():
    file = './part2_sample.txt'
    result = part_2(file)
    assert result == 281


def test_parse_line():
    inputs = [
            'two1nine',
            'eightwothree',
            'abcone2threexyz',
            'xtwone3four',
            '4nineeightseven2',
            'zoneight234',
            '7pqrstsixteen'
            ]
    results = [parse_line(x) for x in inputs]
    assert results[0] == 29
    assert results[1] == 83
    assert results[2] == 13
    assert results[3] == 24
    assert results[4] == 42
    assert results[5] == 14
    assert results[6] == 76
