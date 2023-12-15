use core::panic;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

type Pair = (usize, usize);
type Distance = usize;
fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let universe = parse_input(&input);
    // for factor in vec![1, 10, 100, 100_000_000] {
    for factor in vec![1_000_000] {
        // let old_universe = expand_universe(&universe, factor);
        // let old_galaxy_pairs = galaxy_pairs(&old_universe);
        // print_table(&universe);
        // let galaxy_pairs = galaxy_pairs(&universe);
        let (columns, rows) = expansion_areas(&universe);
        let galaxy_pairs = expanded_galaxy_pairs(&universe, columns, rows, factor);
        // print_pairs(&old_galaxy_pairs);
        // print_pairs(&galaxy_pairs);
        let distances = shortest_distances(&galaxy_pairs);
        // print_distances(&distances);
        println!("part1: {}", part1(&distances));
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect::<Vec<char>>());
    }
    return result;
}

fn expansion_areas(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut columns = vec![usize::MAX];
    let mut rows = vec![usize::MAX];

    for col in 0..universe[0].len() {
        let mut is_empty = true;
        for (row, _) in universe.iter().enumerate() {
            if universe[row][col] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            columns.push(col);
        }
    }

    for row in 0..universe.len() {
        let mut is_empty = true;
        for (col, _) in universe[row].iter().enumerate() {
            if universe[row][col] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            rows.push(row);
        }
    }
    return (columns, rows);
}

fn expand_universe(universe: &Vec<Vec<char>>, factor: usize) -> Vec<Vec<char>> {
    let mut new_universe = universe.clone();
    // print_table(&new_universe);
    let mut empty_cols = 0;
    for col in 0..universe[0].len() {
        let mut is_empty = true;
        for (row, _) in universe.iter().enumerate() {
            if universe[row][col] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            for (row, _) in universe.iter().enumerate() {
                let mut u_row = new_universe[row].clone();
                let actual_col = col + empty_cols;
                u_row.splice(actual_col..actual_col, vec!['.'; factor]);
                new_universe[row] = u_row;
            }
            empty_cols += factor;
        }
    }
    // println!("expand x-axis (factor {}):", factor);
    // print_table(&new_universe);
    let mut empty_rows = 0;
    for row in 0..universe.len() {
        let mut is_empty = true;
        for (col, _) in universe[row].iter().enumerate() {
            if universe[row][col] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            let actual_row = row + empty_rows;
            new_universe.splice(
                actual_row..actual_row,
                vec![vec!['.'; new_universe[row].len()]; factor],
            );
            empty_rows += factor;
        }
    }
    // println!("expand y-axis (factor {}):", factor);
    // print_table(&new_universe);
    return new_universe;
}

fn galaxy_pairs(universe: &Vec<Vec<char>>) -> Vec<Pair> {
    let mut result = Vec::new();
    for (row, line) in universe.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '#' {
                result.push((row, col));
            }
        }
    }
    return result;
}

fn expanded_galaxy_pairs(
    universe: &Vec<Vec<char>>,
    columns: Vec<usize>,
    rows: Vec<usize>,
    factor: usize,
) -> Vec<Pair> {
    let mut result = Vec::new();
    for (row, line) in universe.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '#' {
                let row_offset = rows
                    .iter()
                    .fold(0, |acc, r| if row >= *r { acc + 1 } else { acc })
                    * (if factor == 1 { 1 } else { factor - 1 })
                    + factor;

                let col_offset = columns
                    .iter()
                    .fold(0, |acc, c| if col >= *c { acc + 1 } else { acc })
                    * (if factor == 1 { 1 } else { factor - 1 })
                    + factor;
                result.push((row + row_offset, col + col_offset));
            }
        }
    }
    return result;
}

// 63072008425490-630720
fn print_table(table: &Vec<Vec<char>>) {
    for line in table.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn print_pairs(pairs: &Vec<Pair>) {
    for (row, col) in pairs.iter() {
        print!("({}, {}), ", row, col);
    }
    println!();
}

fn print_distances(distances: &HashMap<Pair, HashMap<Pair, Distance>>) {
    for (x, (pair, distance)) in distances
        .iter()
        .sorted_by_key(|(x, y)| (x.0, x.1))
        .enumerate()
    {
        println!("{}:", x + 1);
        for (y, (neighbor, distance)) in distance
            .iter()
            .sorted_by_key(|(x, y)| (x.0, x.1))
            .enumerate()
        {
            println!(" ->{}: {}, ", y + 1, distance);
        }
        println!();
    }
}

fn manhattan_distance(a: &Pair, b: &Pair) -> Distance {
    return (a.0 as i32 - b.0 as i32).abs() as usize + (a.1 as i32 - b.1 as i32).abs() as usize;
}

fn shortest_distances(pairs: &Vec<Pair>) -> HashMap<Pair, HashMap<Pair, Distance>> {
    let mut unvisited: Vec<Pair> = pairs.clone();
    let mut distances: HashMap<Pair, HashMap<Pair, Distance>> = HashMap::new();
    for (row, col) in pairs.iter() {
        distances.insert(
            (*row, *col),
            pairs.iter().fold(HashMap::new(), |mut acc, (r, c)| {
                if *r == *row && *c == *col {
                    acc.insert((*r, *c), 0);
                } else {
                    acc.insert((*r, *c), usize::MAX);
                }
                acc
            }),
        );
    }
    while unvisited.len() > 0 {
        let node = unvisited.pop().unwrap();
        for neighbor in unvisited.iter() {
            if node == *neighbor {
                continue;
            }
            let distance_node_to_neighbor = distances.get(&node).unwrap().get(neighbor).unwrap();
            let distance_neighbor_to_node = distances
                .get(neighbor)
                .unwrap()
                .get(&node)
                .unwrap_or(&usize::MAX);
            let distance = manhattan_distance(&node, neighbor);
            let min_distance = std::cmp::min(
                distance,
                std::cmp::min(*distance_node_to_neighbor, *distance_neighbor_to_node),
            );
            distances.get_mut(&node).unwrap().remove(neighbor);
            distances
                .get_mut(neighbor)
                .unwrap()
                .insert(node, min_distance);
        }
    }

    return distances;
}

fn part1(distances: &HashMap<Pair, HashMap<Pair, Distance>>) -> i64 {
    let sum = distances.iter().fold(0, |acc, (_node, neighbors)| {
        return acc
            + neighbors.iter().fold(0, |acc, (_neighbor, distance)| {
                return acc + *distance as i64;
            });
    });
    return sum;
}
