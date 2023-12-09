
use serde::{Deserialize, Serialize};
use std::cmp::{min, Ordering};
use std::{io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let hands = parse_lines(&input);
    let hands2 = parse_lines2(&input);
    // println!("{}", serde_json::to_string_pretty(&hands).unwrap());
    println!("{}", serde_json::to_string_pretty(&hands2).unwrap());

    println!("part1: {}", part1(&hands));
    println!("part2: {}", part2(&hands2));
    // println!("part2: {}", part2(race));
    // println!("part2: {}", part2(&seeds, maps));
}

#[derive(Serialize, Deserialize, Eq)]
struct Hand {
    kind: i32,
    cards: Vec<i32>,
    bid: i64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == other.kind {
            for (pos, card) in self.cards.iter().enumerate() {
                if card.clone() == other.cards[pos] {
                    continue;
                } else {
                    return card.cmp(&other.cards[pos]);
                }
            }
        }
        return self.kind.cmp(&other.kind);
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return (self.kind, &self.cards) == (other.kind, &other.cards);
    }
}

fn card_to_value(char: char) -> i32 {
    return match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => char.to_digit(10).unwrap() as i32,
    };
}
fn card_to_value2(char: char) -> i32 {
    return match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => char.to_digit(10).unwrap() as i32,
    };
}

fn cards_to_kind(cards: Vec<i32>) -> i32 {
    let mut holding = vec![0; 16];
    for card in cards {
        holding[card as usize] += 1;
    }
    holding.sort();
    holding.reverse();
    let pairs = &holding[0..2];
    return match (pairs[0], pairs[1]) {
        (5, _) => 7, // five of a kind
        (4, _) => 6, // four of a kind
        (3, 2) => 5, // full house
        (3, _) => 4, // three of a kind
        (2, 2) => 3, // two pair
        (2, _) => 2, // one pair
        _ => 1,      // high card
    };
}
fn cards_to_kind2(cards: Vec<i32>) -> i32 {
    let mut holding = vec![0; 16];
    for card in cards {
        holding[card as usize] += 1;
    }
    let jokers = holding[1];
    // holding[0] = 0;
    let mut held = holding
        .iter()
        .map(|v| min(v + jokers, 5))
        .collect::<Vec<i32>>();
    held[1] = jokers;
    held.sort();
    held.reverse();
    println!("{:?}", held);
    let pairs = &held[0..2];
    return match (pairs[0], pairs[1], jokers) {
        (5, _, _) => 7, // five of a kind
        (4, _, _) => 6, // four of a kind
        (3, 3, 1) => 5, // full house
        (3, 2, 0) => 5, // full house
        (3, _, _) => 4, // three of a kind
        (2, 2, 1) => 2, // one pair
        (2, 2, _) => 3, // two pair
        (2, _, _) => 2, // one pair
        _ => 1,         // high card
    };
}
fn parse_lines(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards: Vec<i32> = parts[0].chars().map(card_to_value).collect();
        let bid = parts[1].parse::<i64>().unwrap();
        let kind = cards_to_kind(cards.clone());
        hands.push(Hand { cards, bid, kind });
    }
    hands.sort();
    return hands;
}
fn parse_lines2(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards: Vec<i32> = parts[0].chars().map(card_to_value2).collect();
        let bid = parts[1].parse::<i64>().unwrap();
        let kind = cards_to_kind2(cards.clone());
        hands.push(Hand { cards, bid, kind });
    }
    hands.sort();
    return hands;
}
fn part1(hands: &Vec<Hand>) -> i64 {
    let mut result = 0;
    for (pos, hand) in hands.iter().enumerate() {
        result += hand.bid * (pos as i64 + 1);
    }
    return result;
}

fn part2(hands: &Vec<Hand>) -> i64 {
    return part1(hands);
}

/*
tries part2:
249975167
250294654
249536327
250294654
250456906
250293474
250563472
251037509
*/
