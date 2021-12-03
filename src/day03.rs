use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("input/03.txt").unwrap();

    let bit_vectors = parse(&contents);
    let gamma_vector = calculate_gamma_vector(&bit_vectors);
    let gamma = bit_vector_to_int(&gamma_vector);
    let epsilon_vector = invert_bit_vector(&gamma_vector);
    let epsilon = bit_vector_to_int(&epsilon_vector);

    let oxygen_rating_vector = find_rating(&bit_vectors, true);
    let oxygen_rating = bit_vector_to_int(&oxygen_rating_vector);
    let scrubber_rating_vector = find_rating(&bit_vectors, false);
    let scrubber_rating = bit_vector_to_int(&scrubber_rating_vector);

    println!("December 03 2021");
    println!("    part1 {:#?}", gamma * epsilon);
    println!("    part2 {:#?}", oxygen_rating * scrubber_rating);
}

fn parse(contents: &String) -> Vec<Vec<bool>> {
    contents
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn calculate_gamma_vector(bit_vectors: &Vec<Vec<bool>>) -> Vec<bool> {
    (0..bit_vectors[0].len())
        .map(|i| get_most_common_bit(bit_vectors, i))
        .collect()
}

fn find_rating(bit_vectors: &Vec<Vec<bool>>, use_common: bool) -> Vec<bool> {
    let mut remaining = bit_vectors.to_vec();
    let mut index = 0;
    while remaining.len() > 1 {
        let common_bit = get_most_common_bit(&remaining, index);
        remaining = remaining
            .into_iter()
            .filter(|bits| use_common == (bits[index] == common_bit))
            .collect();
        index += 1;
    }
    remaining[0].clone()
}

fn get_most_common_bit(bit_vectors: &Vec<Vec<bool>>, index: usize) -> bool {
    let mut ones = 0;
    for bits in bit_vectors {
        if bits[index] {
            ones += 1;
        }
    }
    ones * 2 >= bit_vectors.len()
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
