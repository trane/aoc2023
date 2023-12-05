use core::num;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let (engine_parts, symbols) = parse_lines(&input);

    // println!("{}", serde_json::to_string_pretty(&engine_parts).unwrap());
    // println!("{}", serde_json::to_string_pretty(&symbols).unwrap());
    println!("part1: {}", part1(&symbols, &engine_parts));
    println!("part2: {}", part2(&symbols, &engine_parts));
}

#[derive(Serialize, Deserialize)]
struct EnginePart {
    number: i32,
    surrounding: Vec<(i32, i32)>,
}
#[derive(Serialize, Deserialize)]
struct Symbol {
    symbol: char,
    location: (i32, i32),
}
fn parse_lines(input: &str) -> (Vec<EnginePart>, Vec<Symbol>) {
    let mut engine_parts = Vec::new();
    let mut symbols = Vec::new();
    let mut l = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < chars.len() {
            if chars[i].is_numeric() {
                let mut engine_part = EnginePart {
                    number: chars[i].to_digit(10).unwrap() as i32,
                    surrounding: Vec::new(),
                };
                let mut j = i + 1;
                while j < chars.len() {
                    if chars[j].is_numeric() {
                        j += 1;
                    } else {
                        break;
                    }
                }
                let number = chars[i..j]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                engine_part.number = number;
                let up = l as i32 - 1;
                let down = l as i32 + 1;
                let left = i as i32 - 1;
                let right = j as i32;
                for c in left..(right + 1) {
                    if c < 0 || c > chars.len() as i32 {
                        continue;
                    }
                    if l > 0 {
                        engine_part.surrounding.push((up, c));
                    }
                    engine_part.surrounding.push((down, c));
                }
                if left >= 0 {
                    engine_part.surrounding.push((l, left));
                }
                if right < chars.len() as i32 {
                    engine_part.surrounding.push((l, right));
                }
                engine_parts.push(engine_part);
                i = j - 1;
            } else if chars[i] != '.' {
                let symbol = Symbol {
                    symbol: chars[i],
                    location: (l as i32, i as i32),
                };
                symbols.push(symbol);
            }
            i += 1;
        }
        l += 1;
    }
    return (engine_parts, symbols);
}

fn part1(symbols: &Vec<Symbol>, engine_parts: &Vec<EnginePart>) -> i32 {
    let mut result = 0;
    for part in engine_parts {
        for location in part.surrounding.to_owned() {
            for symbol in symbols {
                let loc = symbol.location;
                if location == loc {
                    // println!(
                    //     "part: {}\nlocation: {},{}",
                    //     part.number, location.0, location.1
                    // );
                    result += part.number;
                }
            }
        }
    }
    return result;
}
fn part2(symbols: &Vec<Symbol>, engine_parts: &Vec<EnginePart>) -> i64 {
    let mut result: i64 = 0;
    for symbol in symbols {
        if symbol.symbol == '*' {
            let mut gears = Vec::new();
            for part in engine_parts {
                for location in part.surrounding.to_owned() {
                    if location == symbol.location {
                        // println!(
                        //     "part: {}\nlocation: {},{}",
                        //     part.number, location.0, location.1
                        // );
                        gears.push(part);
                    }
                }
            }
            if gears.len() > 1 {
                result += gears
                    .iter()
                    .fold(1, |acc: i64, g| acc * g.number.to_owned() as i64);
            }
        }
    }
    return result;
}
