from part1 import part_1, line_parse


def test_main_output():
    file = './part1_sample.txt'
    result = part_1(file)
    assert result == 142


def test_line_parse():
    inputs = [
            '1abc2',
            'pqr3stu8vwx',
            'a1b2c3d4e5f',
            'treb7uchet',
            ]
    outputs = [line_parse(x) for x in inputs]
    assert outputs[0] == 12
    assert outputs[1] == 38
    assert outputs[2] == 15
    assert outputs[3] == 77
