use core::panic;
use std::{collections::HashSet, io::Read};

use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let (start, nodes) = get_nodes(&input);
    // println!(
    //     "({}, {}): {}",
    //     start.0,
    //     start.1,
    //     serde_json::to_string_pretty(&nodes).unwrap()
    // );
    let _graph = make_graph(&nodes, start);
    // let graph = clean_graph(&_graph, start);
    // println!("{}", serde_json::to_string_pretty(&graph).unwrap());

    // println!("part1: {}", part1(&graph));
    // println!("part2: {}", part2(&input, &_graph));
    // println!("part2a: {}", part2a(&input));
    println!("part2b: {}", part2b(&input, &_graph));
}

#[derive(Serialize, Deserialize)]
struct Input {
    chars: Vec<Vec<char>>,
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize)]
struct Graph {
    nodes: Vec<Node>,
}
#[derive(Serialize, Deserialize)]
struct Node {
    value: char,
    location: (usize, usize),
    connections: Vec<(usize, usize)>,
}

fn parse_lines(input: &str) -> Input {
    let mut start = (0, 0);
    return Input {
        chars: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (x, y);
                        }
                        return c;
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
        x: start.0,
        y: start.1,
    };
}
fn parse_lines2(input: &str) -> Input {
    let mut start = (0, 0);
    return Input {
        chars: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (x, y);
                        }
                        return c;
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
        x: start.0,
        y: start.1,
    };
}

fn part2b(input: &str, graph: &Graph) -> i32 {
    let locations = graph
        .nodes
        .iter()
        .map(|node| node.location)
        .collect::<HashSet<(usize, usize)>>();
    let chars = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if locations.contains(&(x, y)) {
                        return match c {
                            '|' | 'F' | '7' => '!',
                            _ => '_',
                        };
                    }
                    return '.';
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    print_table(&chars);
    let mut inside = false;
    let mut result = 0;
    for (y, line) in chars.iter().enumerate() {
        inside = false;
        for (x, c) in line.iter().enumerate() {
            if *c == '.' && inside {
                println!("({}, {})", y, x);
                result += 1;
            }
            if *c == '!' {
                inside = !inside;
            }
        }
    }
    return result;
}

fn part2(iput: &str, graph: &Graph) -> i32 {
    let chars = iput
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let locations = graph
        .nodes
        .iter()
        .map(|node| node.location)
        .collect::<HashSet<(usize, usize)>>();
    let mut new_chars: Vec<Vec<char>> = chars
        .clone()
        .iter()
        .enumerate()
        .map(|(y, line)| {
            return line
                .clone()
                .iter()
                .enumerate()
                .map(|(x, c)| {
                    if locations.contains(&(x, y)) {
                        return 'B';
                    }
                    if y == 0 || y == chars.len() - 1 {
                        return 'O';
                    }
                    if x == 0 || x == line.len() - 1 {
                        return 'O';
                    }
                    if *c != '.' {
                        return '*';
                    }
                    return '.';
                })
                .collect::<Vec<char>>();
        })
        .collect();
    print_table(&new_chars);
    println!("");
    loop {
        let mut changed = false;
        for (y, line) in new_chars.clone().iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if new_chars[y][x] != '.' {
                    continue;
                }
                if new_chars[y - 1][x] == 'O'
                    || new_chars[y + 1][x] == 'O'
                    || new_chars[y][x - 1] == 'O'
                    || new_chars[y][x + 1] == 'O'
                {
                    new_chars[y][x] = 'O';
                    changed = true;
                    continue;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let mut result = 0;
    for line in new_chars.iter() {
        for c in line.iter() {
            if *c == '.' {
                result += 1;
            }
        }
    }
    print_table(&new_chars);
    return result;
}

fn print_table(table: &Vec<Vec<char>>) {
    for line in table.iter() {
        for c in line.iter() {
            if *c == 'O' {
                print!("{}", c.to_string().red());
                continue;
            }
            if *c == '.' {
                print!("{}", "I".green().bold());
                continue;
            }
            print!("{}", c);
        }
        println!();
    }
}

fn get_nodes(input: &str) -> ((usize, usize), Vec<Vec<Node>>) {
    let inpt = parse_lines(input);
    let mut nodes: Vec<Vec<Node>> = Vec::new();
    let mut start = (0, 0);
    for (y, line) in inpt.chars.iter().enumerate() {
        nodes.push(Vec::new());
        for (x, c) in line.iter().enumerate() {
            let value = c.clone();
            let location = (x, y);
            let mut connections = Vec::new();
            if y > 0 {
                let peek = inpt.chars[y - 1][x];
                if is_connected(c.clone(), peek, 'n') {
                    connections.push((x, y - 1));
                }
            }
            if y < inpt.chars.len() - 1 {
                let peek = inpt.chars[y + 1][x];
                if is_connected(c.clone(), peek, 's') {
                    connections.push((x, y + 1));
                }
            }
            if x > 0 {
                let peek = inpt.chars[y][x - 1];
                if is_connected(c.clone(), peek, 'w') {
                    connections.push((x - 1, y));
                }
            }
            if x < inpt.chars[y].len() - 1 {
                let peek = inpt.chars[y][x + 1];
                if is_connected(c.clone(), peek, 'e') {
                    connections.push((x + 1, y));
                }
            }
            if *c == 'S' {
                start = (x, y);
            }
            nodes[y].push(Node {
                value,
                location,
                connections,
            });
        }
    }
    return (start, nodes);
}

fn is_connected(curr: char, peek: char, direction: char) -> bool {
    match direction {
        'n' => match curr {
            'S' | '|' | 'L' | 'J' => match peek {
                'S' | '|' | 'F' | '7' => true,
                _ => false,
            },
            _ => false,
        },
        's' => match curr {
            'S' | '|' | 'F' | '7' => match peek {
                'S' | '|' | 'L' | 'J' => true,
                _ => false,
            },
            _ => false,
        },
        'e' => match curr {
            'S' | '-' | 'F' | 'L' => match peek {
                'S' | '-' | 'J' | '7' => true,
                _ => false,
            },
            _ => false,
        },
        'w' => match curr {
            'S' | '-' | 'J' | '7' => match peek {
                'S' | '-' | 'L' | 'F' => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

fn make_graph(nodes: &Vec<Vec<Node>>, start: (usize, usize)) -> Graph {
    let mut graph = Graph { nodes: Vec::new() };
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);
    while queue.len() > 0 {
        let curr = queue.pop().unwrap();
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);
        if nodes[curr.1][curr.0].connections.len() == 2 {
            for connection in nodes[curr.1][curr.0].connections.clone() {
                queue.push((connection.0, connection.1));
            }
            graph.nodes.push(Node {
                value: nodes[curr.1][curr.0].value,
                location: curr,
                connections: nodes[curr.1][curr.0].connections.clone(),
            });
        }
    }
    return graph;
}

fn clean_graph(graph: &Graph, start: (usize, usize), next: (usize, usize)) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);
    while queue.len() > 0 {
        let curr = queue.pop().unwrap();
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);
        if curr == next {
            return true;
        }
        let node = graph.nodes.iter().find(|n| n.location == curr).unwrap();
        if node.connections.len() > 0 {
            for connection in node.connections.clone() {
                queue.push((connection.0, connection.1));
            }
        }
    }
    return false;
}

// fn _clean_graph(
//     graph: &Graph,
//     visited: &HashSet<(usize, usize)>,
//     start: (usize, usize),
//     end: (usize, usize),
// ) -> HashSet<(usize, usize)> {
//     for
// }
// fn clean_graph(graph: &Graph, start: (usize, usize)) -> Vec<(usize, usize)> {
//     let mut visited: HashSet<(usize, usize)> = HashSet::new();
//     for node in graph.nodes.iter() {
//         for next in node.connections.iter() {
//             if *next == start {
//                 break;
//             }
//             clean_graph(graph, *next);
//         }
//     }
//     queue.push(start);
//     while queue.len() > 0 {
//         let curr = queue.pop().unwrap();
//         if visited.contains(&curr) {
//             continue;
//         }
//         visited.insert(curr);
//         if graph.nodes[curr.1][curr.0].connections.len() > 0 {
//             for connection in graph.nodes[curr.1][curr.0].connections.clone() {
//                 queue.push((connection.0, connection.1));
//             }
//             new_graph.nodes.push(Node {
//                 value: graph.nodes[curr.1][curr.0].value,
//                 location: curr,
//                 connections: graph.nodes[curr.1][curr.0].connections.clone(),
//             });
//         }
//     }
//     return new_graph;
// }
fn part1(graph: &Graph) -> usize {
    let result = if graph.nodes.len() % 2 == 0 {
        graph.nodes.len() / 2
    } else {
        graph.nodes.len() / 2 + 1
    };
    println!("nodes: {}", graph.nodes.len());
    for node in graph.nodes.iter() {
        println!(
            "node: ({}, {}) {}",
            node.location.0, node.location.1, node.value
        );
    }
    return result;
}

fn direction(char: char) -> (i32, i32) {
    match char {
        'S' => (0, -1),
        's' => (0, 1),
        'e' => (1, 0),
        'w' => (-1, 0),
        _ => (0, 0),
    }
}

fn set_start_direction(start: (usize, usize), chars: &Vec<Vec<char>>) -> (char, (usize, usize)) {
    let mut start = start;
    let n = chars[start.1 - 1][start.0];
    let s = chars[start.1 + 1][start.0];
    let w = chars[start.1][start.0 - 1];
    let e = chars[start.1][start.0 + 1];
    match n {
        '|' | 'F' | '7' => {
            return ('n', (start.0, start.1 - 1));
        }
        _ => {}
    }
    match s {
        '|' | 'J' | 'L' => {
            return ('s', (start.0, start.1 + 1));
        }
        _ => {}
    }
    match w {
        '-' | 'F' | 'L' => {
            return ('w', (start.0 - 1, start.1));
        }
        _ => {}
    }
    match e {
        '-' | 'J' | '7' => {
            return ('e', (start.0 + 1, start.1));
        }
        _ => {}
    }
    panic!("uh oh");
}

fn part2a(input: &str) -> i32 {
    let inpt = parse_lines(input);
    let mut chars = inpt.chars.clone();
    let mut start = (inpt.x, inpt.y);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    chars[start.1][start.0] = '*';
    let (mut heading, mut next) = set_start_direction(start, &chars);
    println!("start: {:?}, heading: {}", start, heading);
    loop {
        start = next.clone();
        let char = chars[next.1][next.0].clone();
        if char == '*' {
            break;
        }
        match (heading, char) {
            ('n', '|') => {
                next.1 -= 1;
            }
            ('n', 'F') => {
                next.0 += 1;
                heading = 'e';
            }
            ('n', '7') => {
                next.0 -= 1;
                heading = 'w';
            }
            ('s', '|') => {
                next.1 += 1;
            }
            ('s', 'J') => {
                next.0 -= 1;
                heading = 'w';
            }
            ('s', 'L') => {
                next.0 += 1;
                heading = 'e';
            }
            ('w', '-') => {
                next.0 -= 1;
            }
            ('w', 'F') => {
                next.1 += 1;
                heading = 's';
            }
            ('w', 'L') => {
                next.1 -= 1;
                heading = 'n';
            }
            ('e', '-') => {
                next.0 += 1;
            }
            ('e', 'J') => {
                next.1 -= 1;
                heading = 'n';
            }
            ('e', '7') => {
                next.1 += 1;
                heading = 's';
            }
            _ => {}
        }
    }
    print_table(&chars);
    return -1;
}
