use std::{collections::HashSet, io::Read};

use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    // let (start, nodes) = get_nodes(&input);
    // println!(
    //     "({}, {}): {}",
    //     start.0,
    //     start.1,
    //     serde_json::to_string_pretty(&nodes).unwrap()
    // );
    // let graph = make_graph(&nodes, start);
    // println!("{}", serde_json::to_string_pretty(&graph).unwrap());

    // println!("part1: {}", part1(&graph));
    println!("part2: {}", part2(&input));
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

fn part2(iput: &str, graph: &Graph) -> i32 {
    let mut chars = iput
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let locations = graph
        .nodes
        .iter()
        .map(|node| node.location)
        .collect::<HashSet<(usize, usize)>>();
    let mut enclosed: Vec<Vec<char>> = Vec::new();
    let mut new_chars: Vec<Vec<char>> = chars.clone();
    loop {
        let mut changed = false;
        for (y, line) in new_chars.clone().iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if locations.contains(&(x, y)) {
                    continue;
                }
                let mut peek = if y > 0 { new_chars[y - 1][x] } else { 'O' };
                if peek == 'O' {
                    new_chars[y][x] = 'O';
                    changed = true;
                    continue;
                }
                peek = if y < new_chars.len() - 1 {
                    new_chars[y + 1][x]
                } else {
                    'O'
                };
                if peek == 'O' {
                    new_chars[y][x] = 'O';
                    changed = true;
                    continue;
                }
                peek = if x < new_chars[0].len() - 1 {
                    new_chars[y][x + 1]
                } else {
                    'O'
                };
                if peek == 'O' {
                    new_chars[y][x] = 'O';
                    changed = true;
                    continue;
                }
                peek = if x > 0 { new_chars[y][x - 1] } else { 'O' };
                if peek == 'O' {
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
    print_table(&new_chars);
    let mut result = 0;
    for line in new_chars.iter() {
        for c in line.iter() {
            if *c == '.' {
                result += 1;
            }
        }
    }
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
            if *c == 'S' {
                start = (x, y);
            }
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
        if nodes[curr.1][curr.0].connections.len() > 0 {
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
