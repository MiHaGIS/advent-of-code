const { part2, parse_game } = require('./part2');

test('sums to the correct number', () => {
  expect(part2('./sample.txt')).toBe(2286);
});

test('Games are parsed correctly', () => {
  let line = 'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green';
  expect(parse_game(line)).toEqual({
    id: 1,
    blue: 6,
    red: 4,
    green: 2,
  });
});
