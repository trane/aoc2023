use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{self, to_vec};
use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let cards = parse_lines(&input);

    // println!("{}", serde_json::to_string_pretty(&cards).unwrap());
    println!("part1: {}", part1(&cards));
}

#[derive(Serialize, Deserialize)]
struct Card {
    number: i32,
    picked: HashSet<i32>,
    winners: HashSet<i32>,
}

fn parse_lines(input: &str) -> Vec<Card> {
    let re = Regex::new(r"^Card\s+(?<number>\d+):\s+(?<picks>.+)\s+\|\s+(?<results>.+)$").unwrap();
    let mut cards = Vec::new();
    for line in input.lines() {
        match re.captures(line) {
            None => panic!("Invalid input: {}", line),
            Some(matching) => {
                let number = matching
                    .name("number")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let picked = matching
                    .name("picks")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                let winners = matching
                    .name("results")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                cards.push(Card {
                    number: number,
                    picked: picked,
                    winners: winners,
                });
            }
        }
    }
    return cards;
}

fn part1(cards: &Vec<Card>) -> i32 {
    let mut result = 0;
    let base: i32 = 2;
    for card in cards {
        let common = card.picked.intersection(&card.winners).count();
        // if common > 1 {}
        if common == 0 {
            continue;
        }
        let value = base.pow(common as u32 - 1);
        // println!(
        //     "Card {}: {} - {}",
        //     card.number,
        //     common,
        //     value,
        //     // serde_json::to_string_pretty(&(int.collect::<Vec<_>>())).unwrap()
        // );
        result += value;
    }
    return result;
}
