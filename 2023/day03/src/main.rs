use std::{fs::read_to_string, thread::current};

fn main() {
    // let result = part1("./sample.txt");
    let result = part1("./inputs.txt");
    println!("Part 1: {}", result);

    let result2 = part2("./inputs.txt");
    println!("Part 2: {}", result2);
}

#[derive(Debug)]
struct Symbol{
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Gear{
    row: usize,
    column: usize,
    parts: Vec<u32>,
}

fn collect_symbols(data: &Vec<String>) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    for (row, line) in data.iter().enumerate() {
        for(column, x) in line.chars().into_iter().enumerate() {
            if x != '.' && !x.is_digit(10){
                symbols.push(Symbol { row, column });
            }
        }
    }
    return symbols
}

fn is_near_symbol(symbols: &Vec<Symbol>, row: usize, column: usize) -> bool {
    for x in column.saturating_sub(1)..=column+1 {
        for y in row.saturating_sub(1)..=row+1 {
            for sym in symbols {
                if x == sym.column && y == sym.row {
                    return true;
                }
            }
        }
    }
    return false;
}

fn part1(file: &str) -> u32 {
    let lines: Vec<String> = read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut total: u32 = 0;
    let symbols = collect_symbols(&lines);

    for (row, line) in lines.iter().enumerate() {
            let mut num: u32 = 0;
            let mut is_continued = false;
            let mut add_to_total = false;
        for (column, item) in line.chars().enumerate() {
            if item.is_digit(10) {
                if is_continued {
                    num = (num * 10) + item.to_digit(10).unwrap();
                }
                else {
                    num = item.to_digit(10).unwrap();
                    is_continued = true;
                }
                is_continued = true;
                add_to_total = add_to_total || is_near_symbol(&symbols, row, column);
            }
            else {
                if add_to_total {
                    total += num;
                }
                is_continued = false;
                num = 0;
                add_to_total = false;
            }
        }
        if add_to_total {
            total += num;
        }
        is_continued = false;
        num = 0;
        add_to_total = false;
    }
    return total;
}

fn collect_gears(data: &Vec<String>) -> Vec<Gear> {
    let mut gears = Vec::<Gear>::new();
    for (row, line) in data.iter().enumerate() {
        for(column, x) in line.chars().into_iter().enumerate() {
            if x == '*'{
                gears.push(Gear { row, column, parts: vec![] });
            }
        }
    }
    return gears
}

fn retrive_gear(gears: &Vec<Gear>, row: usize, column: usize) -> Option<usize> {
    for x in column.saturating_sub(1)..=column+1 {
        for y in row.saturating_sub(1)..=row+1 {
            for (i, gear) in gears.iter().enumerate() {
                if x == gear.column && y == gear.row {
                    return Some(i);
                }
            }
        }
    }
    return None;
}

fn part2(file: &str) -> u32 {
    let lines: Vec<String> = read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut gears = collect_gears(&lines);

    for (row, line) in lines.iter().enumerate() {
        let mut current_num = 0;
        let mut connected_gear_index: Option<usize> = None;
        for (column, item) in line.chars().enumerate() {
            if item.is_digit(10) {
                current_num = (current_num * 10) + item.to_digit(10).unwrap();
                connected_gear_index = match connected_gear_index {
                    Some(_) => connected_gear_index,
                    None => retrive_gear(&gears, row, column),
                };
            } else {
                if connected_gear_index.is_some() {
                    gears[connected_gear_index.unwrap()].parts.push(current_num);
                };
                current_num = 0;
                connected_gear_index = None;
            }
        }
        if connected_gear_index.is_some() {
            gears[connected_gear_index.unwrap()].parts.push(current_num);
        };
        current_num = 0;
        connected_gear_index = None;
    }
    let mut total = 0;
    for gear in gears {
        if gear.parts.len() == 2 {
            total += gear.parts[0] * gear.parts[1];
        }
    }
    return total;
}

