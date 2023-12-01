use regex::Regex;
use serde_json::json;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let parsed = parse(&input);
    // println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

pub fn part1(parsed: &Vec<String>) -> i32 {
    let mut result = 0;
    for line in parsed {
        let mut line_numbers: Vec<char> = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {
                line_numbers.push(character);
            }
        }
        let mut num = String::new();
        num += &line_numbers.first().unwrap().to_string();
        num += &line_numbers.last().unwrap().to_string();
        result += num.parse::<i32>().unwrap();
    }
    return result;
}

pub fn writtentonum(input: &str) -> char {
    return match input {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("Invalid input to writtentonum: {}", input),
    };
}

pub fn part2(parsed: &Vec<String>) -> i32 {
    let re = Regex::new(r"^(?:one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut result = 0;
    for line in parsed {
        let mut line_numbers: Vec<char> = Vec::new();
        let mut i = 0;
        while i < line.chars().count() {
            let character = line.chars().nth(i).unwrap();
            if character.is_numeric() {
                line_numbers.push(character);
                i += 1;
            } else {
                match re.find(&line[i..]) {
                    Some(matching) => {
                        let match_str = matching.as_str();
                        line_numbers.push(writtentonum(match_str));
                        i += match_str.len() - 1;
                    }
                    None => {
                        i += 1;
                    }
                }
            }
        }
        let mut num = String::new();
        num += &line_numbers.first().unwrap().to_string();
        num += &line_numbers.last().unwrap().to_string();
        result += num.parse::<i32>().unwrap();
        let obj = json!({
            "line": line,
            "num": num,
            "numbers": line_numbers,
            "result": result
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    }
    return result;
}

pub fn parse(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    return result;
}
