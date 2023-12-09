use std::io::Read;

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let reports = parse_lines(&input);
    // println!("{}", serde_json::to_string_pretty(&map_helper).unwrap());

    println!("part1: {}", part1(&reports));
    println!("part2: {}", part2r(&reports));
}

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
}

fn part1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for report in reports {
        let mut analysis: Vec<Vec<i32>> = Vec::new();
        analysis.push(report.clone());
        loop {
            let mut a = Vec::new();
            let d = analysis.last().unwrap();
            for (pos, next) in d.iter().enumerate() {
                // println!("{} {}", pos, next);
                if pos == d.len() - 1 {
                    break;
                }
                a.push(d[pos + 1] - next);
            }
            analysis.push(a.clone());
            if a.iter().all(|v| *v == 0) {
                break;
            }
        }
        result += analysis
            .iter()
            .rfold(0, |acc, v| return acc + v.last().unwrap());
    }
    return result;
}

fn part2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for report in reports {
        let mut analysis: Vec<Vec<i32>> = Vec::new();
        analysis.push(report.clone());
        loop {
            let mut a = Vec::new();
            let d = analysis.last().unwrap();
            for (pos, next) in d.iter().enumerate() {
                // println!("{} {}", pos, next);
                if pos == d.len() - 1 {
                    break;
                }
                a.push(d[pos + 1] - next);
            }
            analysis.push(a.clone());
            if a.iter().all(|v| *v == 0) {
                break;
            }
        }
        result += analysis
            .iter()
            .rfold(0, |acc, v| return v.first().unwrap() - acc);
    }
    return result;
}

fn part2r(reports: &Vec<Vec<i32>>) -> i32 {
    return reports.iter().fold(0, |result, report| {
        let analysis = itertools::unfold(vec![report.clone()], |acc| {
            let a = acc.last().unwrap().iter().enumerate().fold(
                Some(vec![]),
                |mut acc: Option<Vec<i32>>, (pos, next)| {
                    if pos == acc.as_ref().unwrap().len() - 1 {
                        return None;
                    }
                    let value = acc.as_ref().unwrap()[pos + 1] - next;
                    acc.as_mut().unwrap().push(value.clone());
                    return Some(acc.unwrap());
                },
            );
            match a {
                Some(a) => {
                    if a.iter().all(|v| *v == 0) {
                        return None;
                    }
                    return Some(a);
                }
                None => return None,
            }
        });
        return 0;
    });
}
