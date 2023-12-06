use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{self, map, to_vec};
use std::{
    collections::{btree_map::Range, HashMap, HashSet},
    io::Read,
    result,
};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let maps = parse_lines(&input);

    // println!("{}", serde_json::to_string_pretty(&cards).unwrap());
    println!("part1: {}", part1(&maps));
    // println!("part2: {}", part2(&cards));
}

struct Input {
    seeds: Vec<i32>,
    maps: Vec<Map>,
}

struct Map {
    name: String,
    source_range: Vec<std::ops::Range<i32>>,
    dest_range: Vec<std::ops::Range<i32>>,
}

fn parse_lines(input: &str) -> Vec<Map> {
    let mut result = Vec::new();
    let seed_re = Regex::new(r"^seeds: (?<seeds>.+)$").unwrap();
    let map_re = Regex::new(r"^(?<name>) map:$").unwrap();
    let map_values =
        Regex::new(r"^(?<dst_start>\d+)\s+(?<src_start>\d+)\s+(?<range>\d+)$").unwrap();
    for line in input.lines() {
        match seed_re.captures(line) {
            Some(captures) => {
                let seeds = captures
                    .name("seeds")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                continue;
            }
            None => {}
        }
        match map_re.captures(line) {
            Some(captures) => {
                let name = captures.name("name").unwrap().as_str().to_string();
                let mut source_range = Vec::new();
                let mut dest_range = Vec::new();
                result.push(Map {
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
                    .parse::<i32>()
                    .unwrap();
                let src_start = captures
                    .name("src_start")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let range = captures
                    .name("range")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                result[result.len() - 1]
                    .source_range
                    .push(src_start..(src_start + range));
                result[result.len() - 1]
                    .dest_range
                    .push(dst_start..(dst_start + range));
                continue;
            }
            None => {}
        }
    }
    return result;
}

fn source_to_dest(map: Map, source: i32) -> i32 {
    for (pos, range) in map.source_range.iter().enumerate() {
        if range.contains(&source) {
            return map.dest_range[pos].start + (source - range.start);
        }
    }
    return source;
}

fn part1(input: &Input) -> i32 {
    let mut locations = Vec::new();
    for seed in input.seeds {
        let dest = seed;
        for map in input.maps {
            let dest = source_to_dest(map, seed);
        }
        locations.push(dest);
    }
    println!("{}", serde_json::to_string_pretty(&locations).unwrap());
    return locations.iter().min().unwrap().clone();
}
