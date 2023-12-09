
use serde::{Deserialize, Serialize};
use std::{io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let races = parse_lines(&input);
    let race = parse_lines2(&input);
    println!("{}", serde_json::to_string_pretty(&race).unwrap());

    println!("part1: {}", part1(&races));
    println!("part2: {}", part2(race));
    // println!("part2: {}", part2(&seeds, maps));
}

#[derive(Serialize, Deserialize)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_lines(input: &str) -> Vec<Race> {
    let mut races = Vec::new();
    let times: Vec<&str> = input.lines().nth(0).unwrap().split_whitespace().collect();
    let distances: Vec<&str> = input.lines().nth(1).unwrap().split_whitespace().collect();
    for (pos, time) in times.iter().enumerate() {
        if pos == 0 {
            continue;
        }
        let race = Race {
            time: time.parse::<i64>().unwrap(),
            distance: distances[pos].parse::<i64>().unwrap(),
        };
        races.push(race);
    }
    return races;
}
fn parse_lines2(input: &str) -> Race {
    let times: Vec<&str> = input.lines().nth(0).unwrap().split_whitespace().collect();
    let distances: Vec<&str> = input.lines().nth(1).unwrap().split_whitespace().collect();
    let mut time_str = String::new();
    let mut distance_str = String::new();
    for (pos, time) in times.iter().enumerate() {
        if pos == 0 {
            continue;
        }
        time_str += time;
        distance_str += distances[pos];
    }
    return Race {
        time: time_str.parse::<i64>().unwrap(),
        distance: distance_str.parse::<i64>().unwrap(),
    };
}

fn part1(races: &Vec<Race>) -> i64 {
    let _result = 1;
    let mut distances = Vec::new();
    for race in races {
        let mut ways = 0;
        for held in 0..race.time {
            let speed = held.clone();
            let remaining_time = race.time - held;
            let distance = speed * remaining_time;
            if distance > race.distance {
                ways += 1;
            }
        }
        distances.push(ways);
    }
    println!("{:?}", distances);
    return distances.iter().product();
}
fn part2(race: Race) -> i64 {
    let _distances = 0..0;
    let mut start = 0;
    let mut end = 0;
    for held in 0..race.time {
        let speed = held.clone();
        let remaining_time = race.time - held;
        let distance = speed * remaining_time;
        if distance > race.distance {
            start = held;
            break;
        }
    }
    for held in (start..race.time).rev() {
        let speed = held.clone();
        let remaining_time = race.time - held;
        let distance = speed * remaining_time;
        if distance > race.distance {
            end = held;
            break;
        }
    }
    println!("start: {}; end: {}", start, end);
    return (start..=end).count() as i64;
}
