use regex::Regex;
use serde::{Deserialize, Serialize};

use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let cards = parse_lines(&input);

    // println!("{}", serde_json::to_string_pretty(&cards).unwrap());
    println!("part1: {}", part1(&cards));
    println!("part2: {}", part2(&cards));
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
        if common == 0 {
            continue;
        }
        let value = base.pow(common as u32 - 1);
        result += value;
    }
    return result;
}

fn part2(cards: &Vec<Card>) -> i32 {
    let mut result = 0;
    let mut card_counts_map = HashMap::new();
    let mut card_winners_map = HashMap::new();
    // result += cards.len() as i32;
    for card in cards {
        let common = card.picked.intersection(&card.winners).count();
        card_winners_map.insert(card.number, common as i32);
        // result += 1 + common as i32;
        let c = card_counts_map
            .entry(card.number)
            .and_modify(|v| *v += 1)
            .or_insert(1)
            .clone();
        // println!(
        //     "card: {}; copies: {}",
        //     card.number,
        //     card_counts_map.get(&card.number).unwrap(),
        // );
        let from: i32 = card.number + 1;
        let until = common as i32 + card.number + 1;
        for _i in 0..c {
            for card_number in from..until {
                let _count = card_counts_map
                    .entry(card_number)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                // println!("card_number: {}; {}", card_number, count);
            }
        }
        result += card_counts_map.get(&card.number).unwrap();
    }
    // println!(
    //     "{}",
    //     serde_json::to_string_pretty(&card_counts_map).unwrap()
    // );
    return result;
}
