use self::Command::{Down, Forward, Up};
use std::{fmt::Debug, fs};

pub fn solve() {
    let contents = fs::read_to_string("input/02.txt").unwrap();

    let commands = parse(&contents);
    let position = calculate_position(&commands);
    let aimed_position = calculate_aimed_position(&commands);

    println!("December 02 2021");
    println!("    part1 {:#?}", position);
    println!("    part2 {:#?}", aimed_position);
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse(contents: &String) -> Vec<Command> {
    contents
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| parse_line(line).unwrap())
        .collect()
}

fn parse_line(line: &str) -> Result<Command, &'static str> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
        return Err("Too many values in a line");
    }
    let value = parts[1]
        .parse::<i32>()
        .map_err(|_| "Second value must be an int")?;
    match parts[0] {
        "forward" => Ok(Forward(value)),
        "down" => Ok(Down(value)),
        "up" => Ok(Up(value)),
        _ => Err("Unknown command"),
    }
}

fn calculate_position(commands: &Vec<Command>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Forward(x) => horizontal += x,
            Down(x) => depth += x,
            Up(x) => depth -= x,
        }
    }
    horizontal * depth
}

fn calculate_aimed_position(commands: &Vec<Command>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Forward(x) => {
                horizontal += x;
                depth += x * aim;
            }
            Down(x) => aim += x,
            Up(x) => aim -= x,
        }
    }
    horizontal * depth
}
