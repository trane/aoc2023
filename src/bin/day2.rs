use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let parsed = parse(&input);
    let result = part1(&parsed);
    println!("{}", result);
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

pub fn parse(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    return result;
}
