use std::{fs::read_to_string};

fn main() {
    // let result = part1("./sample.txt");
    let result = part1("./inputs.txt");
    println!("{}", result);
}

#[derive(Debug)]
struct Symbol{
    row: usize,
    column: usize
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
    println!("Request: row {}, col {}", row, column);
    for x in column.saturating_sub(1)..=column+1 {
        for y in row.saturating_sub(1)..=row+1 {
            println!("Scanning: row: {}, col: {}", y, x);
            for sym in symbols {
                if x == sym.column && y == sym.row {
                    println!("HIT: row {}, col {}", row, column);
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
    println!("{:?}", symbols);

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
                println!("Add: {}", add_to_total);
            }
            else {
                println!("Number: {}", num);
                if add_to_total {
                    total += num;
                    println!("Total: {}", total);
                }
                is_continued = false;
                num = 0;
                add_to_total = false;
            }
        }
        println!("Number: {}", num);
        if add_to_total {
            total += num;
            println!("Total: {}", total);
        }
        is_continued = false;
        num = 0;
        add_to_total = false;
    }
    return total;
}


