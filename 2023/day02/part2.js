const fs = require('fs');

const is_valid_game = (game) => {
  return (game.blue <= 14) && (game.green <= 13) && (game.red <= 12);
}

const parse_game = (line) => {
  const id = parseInt(line.slice(5).split(':')[0]);
  const blue = [...line.matchAll(/(\d+) blue/g)]
    .reduce((acc, value) => { return Math.max(acc, parseInt(value[1])) },
      0
    );
  const red = [...line.matchAll(/(\d+) red/g)]
    .reduce((acc, value) => { return Math.max(acc, parseInt(value[1])) },
      0
    );
  const green = [...line.matchAll(/(\d+) green/g)]
    .reduce((acc, value) => { return Math.max(acc, parseInt(value[1])) },
      0
    );
  return {
    id,
    blue,
    green,
    red,
  };
}

const part2 = (file) => {
  const data = fs.readFileSync(file, encoding='ascii').split('\n');
  data.pop();
  const games = data.map((line) => parse_game(line));
  const result = games.reduce(
    (accumulator, game) => {
        return accumulator + ( game.red * game.green * game.blue);
    },
    0
  );

  return result;
}

const main = () => {
  console.log({answer: part2('./inputs.txt')});
}


main();

module.exports = {
  part2,
  parse_game,
  is_valid_game,
};
