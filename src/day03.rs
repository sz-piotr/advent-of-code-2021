use core::panic;
use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("input/03.txt").unwrap();

    let bits = parse(&contents);
    let gamma_vector = calculate_gamma_vector(&bits);
    let gamma = bit_vector_to_int(&gamma_vector);
    let epsilon_vector = invert_bit_vector(&gamma_vector);
    let epsilon = bit_vector_to_int(&epsilon_vector);

    println!("December 03 2021");
    println!("    part1 {:#?}", gamma * epsilon);
}

fn parse(contents: &String) -> Vec<Vec<bool>> {
    contents
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn calculate_gamma_vector(bits: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut totals = vec![0; bits[0].len()];
    for item in bits {
        if item.len() != totals.len() {
            panic!("Vector length mismatch!");
        }
        for (i, bit) in item.iter().enumerate() {
            if *bit {
                totals[i] += 1;
            }
        }
    }
    let half_len = bits.len() / 2;
    totals.iter().map(|count| *count > half_len).collect()
}

fn invert_bit_vector(bits: &Vec<bool>) -> Vec<bool> {
    bits.iter().map(|b| !b).collect()
}

fn bit_vector_to_int(bits: &Vec<bool>) -> i32 {
    let mut power = 1;
    let mut value = 0;
    for bit in bits.iter().rev() {
        if *bit {
            value += power;
        }
        power *= 2;
    }
    value
}
