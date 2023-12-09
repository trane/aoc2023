use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let (seeds, maps) = parse_lines(&input);

    println!("part1: {}", part1(&seeds, &maps));
    println!("part2: {}", part2(&seeds, maps));
}

#[derive(Serialize, Deserialize)]
struct Map {
    name: String,
    source_range: Vec<std::ops::Range<i64>>,
    dest_range: Vec<std::ops::Range<i64>>,
}

fn parse_lines(input: &str) -> (Vec<i64>, Vec<Map>) {
    let mut results = Vec::new();
    let mut seeds = Vec::new();
    let seed_re = Regex::new(r"^seeds:\s+(?<seeds>.+)$").unwrap();
    let map_re = Regex::new(r"^(?<name>.+)\s+map:$").unwrap();
    let map_values =
        Regex::new(r"^(?<dst_start>\d+)\s+(?<src_start>\d+)\s+(?<range>\d+)$").unwrap();
    for line in input.lines() {
        match seed_re.captures(line) {
            Some(captures) => {
                seeds = captures
                    .name("seeds")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                continue;
            }
            None => {}
        }
        match map_re.captures(line) {
            Some(captures) => {
                let name = captures.name("name").unwrap().as_str().to_string();
                let source_range = Vec::new();
                let dest_range = Vec::new();
                results.push(Map {
                    name: name,
                    source_range: source_range,
                    dest_range: dest_range,
                });
                continue;
            }
            None => {}
        }
        match map_values.captures(line) {
            Some(captures) => {
                let dst_start = captures
                    .name("dst_start")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                let src_start = captures
                    .name("src_start")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                let range = captures
                    .name("range")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                results
                    .last_mut()
                    .map(|m| m.source_range.push(src_start..(src_start + range)));
                results
                    .last_mut()
                    .map(|m| m.dest_range.push(dst_start..(dst_start + range)));
                continue;
            }
            None => {}
        }
    }
    return (seeds, results);
}

fn source_to_dest(map: &Map, source: i64) -> i64 {
    for (pos, range) in map.source_range.iter().enumerate() {
        if range.contains(&source) {
            let value = map.dest_range[pos].start + (source - range.start);
            return value;
        }
    }
    return source.clone();
}
fn seed_to_location(seed: i64, maps: &Vec<Map>) -> i64 {
    return maps.iter().fold(seed, |acc, map| source_to_dest(map, acc));
}

fn part1(seeds: &Vec<i64>, maps: &Vec<Map>) -> i64 {
    let mut locations = Vec::new();
    for seed in seeds {
        locations.push(seed_to_location(*seed, maps));
    }
    // println!("{}", serde_json::to_string_pretty(&locations).unwrap());
    return locations.iter().min().unwrap().clone();
}

fn seed_ranges(seeds: &Vec<i64>) -> Vec<std::ops::Range<i64>> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        ranges.push(seeds[i]..(seeds[i] + seeds[i + 1]));
        i += 2;
    }
    return ranges;
}

fn work(seed_range: std::ops::Range<i64>, maps: &Vec<Map>) -> i64 {
    let mut lowest = i64::MAX;
    for seed in seed_range {
        let cur = seed_to_location(seed, maps);
        if cur < lowest {
            lowest = cur;
        }
    }
    return lowest;
}

fn part2(seeds: &Vec<i64>, maps: Vec<Map>) -> i64 {
    let seed_ranges = seed_ranges(seeds);
    let mut lowest = i64::MAX;
    for range in seed_ranges {
        println!("starting range: {}..{}", range.start, range.end,);
        let cur = work(range, &maps);
        println!("...finished! cur: {}, lowest: {}", cur, lowest);
        if cur < lowest {
            lowest = cur;
        }
    }
    return lowest;
}
