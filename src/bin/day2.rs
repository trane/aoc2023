use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let parsed = parse_games(&input);
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

struct Bag {
    blue: i32,
    green: i32,
    red: i32,
}

#[derive(Serialize, Deserialize)]
struct Game {
    number: i32,
    blues: Vec<i32>,
    greens: Vec<i32>,
    reds: Vec<i32>,
}

fn parse_games(input: &str) -> Vec<Game> {
    let re: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let mut games = Vec::new();
    for line in input.lines() {
        match re.captures(line) {
            Some(captures) => {
                let game_number = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let game_reveals = captures.get(2).unwrap().as_str();
                let mut game = Game {
                    number: game_number,
                    blues: Vec::new(),
                    greens: Vec::new(),
                    reds: Vec::new(),
                };
                for reveal in game_reveals.split("; ").collect::<Vec<&str>>() {
                    for cubes in reveal.split(", ").collect::<Vec<&str>>() {
                        let counts = cubes.split(" ").collect::<Vec<&str>>();
                        let count = counts.first().unwrap().parse::<i32>().unwrap();
                        let cube_color = counts.last().unwrap();
                        match cube_color.to_string().as_str() {
                            "blue" => game.blues.push(count),
                            "green" => game.greens.push(count),
                            "red" => game.reds.push(count),
                            _ => panic!("Invalid cube color: {}", cube_color),
                        }
                    }
                }
                games.push(game)
            }
            None => panic!("Invalid input: {}", line),
        }
    }
    return games;
}

fn part1(games: &Vec<Game>) -> i32 {
    let bag = Bag {
        blue: 14,
        green: 13,
        red: 12,
    };
    // let mut possible_games: Vec<Game> = Vec::new();
    let mut result = 0;
    for game in games {
        let blue = game.blues.iter().max().unwrap();
        let red = game.reds.iter().max().unwrap();
        let green = game.greens.iter().max().unwrap();
        if (bag.blue - blue) >= 0 && (bag.red - red) >= 0 && (bag.green - green) >= 0 {
            result += game.number;
        }
    }
    return result;
}

fn part2(games: &Vec<Game>) -> i32 {
    let mut result = 0;
    for game in games {
        let blue = game.blues.iter().max().unwrap();
        let red = game.reds.iter().max().unwrap();
        let green = game.greens.iter().max().unwrap();
        result += blue * red * green;
    }
    return result;
}
