use std::fs::read_to_string;
use std::str::FromStr;

fn part1(file: &str) -> usize {
    let lines: Vec<String> = read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut sum: usize = 0;
    for line in lines {
        let card = Sratcher::from_str(line.as_str()).unwrap();
        sum += card.get_score();
    }
    return sum;
}

fn part2(file: &str) -> i32 {
    let lines: Vec<String> = read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let cards: Vec<Sratcher> = lines
        .iter()
        .map(|x| Sratcher::from_str(x.as_str()).unwrap())
        .collect();
    let mut count = 0;
    for c in cards.iter() {
        count += c.collect_winners(&cards);
    }
    return count;
}

struct Sratcher {
    game_num: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
}

impl Sratcher {
    fn get_score(&self) -> usize {
        let matches = self.winning_numbers
            .iter()
            .filter(
                |i| self.card_numbers.contains(i)
            )
            .count() as u32;
        match matches {
            0 => 0,
            _ => usize::pow(2, matches - 1)
        }
    }

    fn collect_winners(&self, collection: &Vec<Sratcher>) -> i32 {
        let matches = self.winning_numbers
            .iter()
            .filter(
                |i| self.card_numbers.contains(i)
            )
            .count();
        let mut cards = 1;
        for index in self.game_num..=self.game_num+matches as i32 - 1 as i32 {
            cards += collection[index as usize].collect_winners(collection)
        }
        return cards;
    }
}

impl FromStr for Sratcher {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").collect();
        let card_info: Vec<&str> = parts[0].split(" ").collect();
        let game_num = card_info.last().unwrap().parse::<i32>().unwrap();

        let numbers: Vec<&str> = parts[1].split("|").collect();

        let winning_numbers: Vec<i32> = numbers[0]
            .trim()
            .split(" ")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        
        let card_numbers: Vec<i32> = numbers[1]
            .trim()
            .split(" ")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        Ok(Sratcher {
            game_num,
            winning_numbers,
            card_numbers,
        })

    }
}



fn main() {
    println!("Welcome to day 4");

    let part1_result = part1("./input.txt");
    println!("Part1: {}", part1_result);

    let part2_result = part2("./input.txt");
    println!("Part2: {}", part2_result);
}
