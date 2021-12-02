use std::fs;

pub fn solve() {
    let contents =
        fs::read_to_string("input/01.txt").unwrap();
    let items = parse(&contents);

    let basic_increases = count_increases(&items);
    let windows = group_into_windows(&items, 3);
    let window_increases = count_increases(&windows);

    println!("December 01 2021");
    println!("    part1 {:#?}", basic_increases);
    println!("    part2 {:#?}", window_increases);
}

fn parse(contents: &String) -> Vec<i32> {
    contents
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn group_into_windows(data: &Vec<i32>, window_size: usize) -> Vec<i32> {
    (0..(data.len() - window_size + 1))
        .map(|i| (0..window_size).fold(0, |sum, j| sum + data[i + j]))
        .collect()
}

fn count_increases(data: &Vec<i32>) -> i32 {
    let mut total = 0;
    let mut previous = 0;
    let mut first = true;
    for current in data {
        if !first && *current > previous {
            total += 1;
        }
        previous = *current;
        first = false;
    }
    return total;
}
