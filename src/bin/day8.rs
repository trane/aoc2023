use regex::Regex;
use serde::{Deserialize, Serialize};


use std::collections::HashMap;
use std::{io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let map_helper = parse_lines(&input);
    // println!("{}", serde_json::to_string_pretty(&map_helper).unwrap());

    // println!("part1: {}", part1(&map_helper));
    // println!("part2: {}", part2(&map_helper));
    println!("part2a: {}", part2a(&map_helper));
}

#[derive(Serialize, Deserialize)]
struct MapHelper {
    map: HashMap<String, (String, String)>,
    directions: String,
    a_s: Vec<String>,
}

fn parse_lines(input: &str) -> MapHelper {
    let re = Regex::new(r"^(?<node>.+)\s+=\s+\((?<left>.+),\s+(?<right>.+)\)$").unwrap();
    let directions = input.lines().take(1).next().unwrap().to_string();
    let mut map = HashMap::new();
    let mut a_s = Vec::new();
    for line in input.lines().skip(2) {
        match re.captures(line) {
            None => panic!("Invalid input: {}", line),
            Some(matching) => {
                let node = matching.name("node").unwrap().as_str().to_string();
                let left = matching.name("left").unwrap().as_str().to_string();
                let right = matching.name("right").unwrap().as_str().to_string();
                if node.ends_with('A') {
                    a_s.push(node.clone());
                }
                map.insert(node, (left, right));
            }
        }
    }
    return MapHelper {
        map: map,
        directions: directions,
        a_s: a_s,
    };
}
fn part1(map_helper: &MapHelper) -> i64 {
    let steps = map_helper.directions.chars();
    let end = "ZZZ";
    let mut start = "AAA";
    let mut results = 0;
    loop {
        for step in steps.clone() {
            results += 1;
            match step {
                'L' => {
                    start = map_helper.map.get(start).unwrap().0.as_str();
                }
                'R' => {
                    start = map_helper.map.get(start).unwrap().1.as_str();
                }
                _ => panic!("Invalid step: {}", step),
            }
            if start == end {
                return results;
            }
        }
    }
}

fn part2(map_helper: &MapHelper) -> i64 {
    let mut steps = 0;
    let mut starts = map_helper.a_s.clone();
    loop {
        for step in map_helper.directions.chars() {
            println!("step: {}", steps);
            steps += 1;
            starts = starts
                .iter()
                .map(|a| {
                    return match step {
                        'L' => map_helper.map.get(a.as_str()).unwrap().0.as_str(),
                        'R' => map_helper.map.get(a.as_str()).unwrap().1.as_str(),
                        _ => panic!("Invalid step: {}", step),
                    };
                })
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            println!("   starts: {:?}", starts);
            if starts.iter().all(|s| s.ends_with('Z')) {
                return steps;
            }
        }
    }
}

fn part2a(map_helper: &MapHelper) -> i64 {
    let mut lengths: Vec<i64> = Vec::new();
    for a in map_helper.a_s.clone() {
        let mut start = a.as_str();
        let mut results = 0;
        'outer: loop {
            for step in map_helper.directions.chars().clone() {
                results += 1;
                match step {
                    'L' => {
                        start = map_helper.map.get(start).unwrap().0.as_str();
                    }
                    'R' => {
                        start = map_helper.map.get(start).unwrap().1.as_str();
                    }
                    _ => panic!("Invalid step: {}", step),
                }
                if start.ends_with('Z') {
                    lengths.push(results);
                    break 'outer;
                }
            }
        }
    }
    return lengths.iter().fold(1, |x, y| num::integer::lcm(x, *y));
}
