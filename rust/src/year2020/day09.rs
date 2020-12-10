use std::collections::HashSet;
use std::io::{stdin, Read};

const PREAMBLE_LENGTH: usize = 25;

fn get_possible_sums(input: &Vec<u64>, index: usize) -> HashSet<u64> {
    let preamble = &input[index-PREAMBLE_LENGTH..index];

    let possible_sums: HashSet<u64> = preamble
        .iter()
        .enumerate()
        .map(|num1| {
            preamble[0..num1.0]
                .iter()
                .enumerate()
                .filter_map(|num2| {
                    if num1.0 != num2.0 {
                        Some(num1.1 + num2.1)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<u64>>()
        })
        .flatten()
        .collect();

    possible_sums
}

pub fn run() {
    let mut input_str = String::new();
    stdin().read_to_string(&mut input_str).unwrap();

    let input: Vec<u64> = input_str
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .collect();

    let numbers = &input[PREAMBLE_LENGTH..];

    let invalid = numbers
        .iter()
        .enumerate()
        .find(|&num| !get_possible_sums(&input, num.0 + PREAMBLE_LENGTH).contains(num.1))
        .unwrap();

    println!("{}", invalid.1);
}
