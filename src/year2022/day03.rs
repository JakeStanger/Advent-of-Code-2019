use std::collections::HashSet;
use std::io::{stdin, BufRead};

const LOWER_OFFSET: u32 = 96;
const UPPER_OFFSET: u32 = 38;

fn priority(char: char) -> u32 {
    char as u32
        - if !char.is_uppercase() {
            LOWER_OFFSET
        } else {
            UPPER_OFFSET
        }
}

pub fn part1() -> u32 {
    stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            Some(line.split_at(line.len() / 2))
                .map(|(a, b)| {
                    (
                        a.chars().collect::<HashSet<char>>(),
                        b.chars().collect::<HashSet<char>>(),
                    )
                })
                .unwrap()
        })
        .map(|(a, b)| *a.intersection(&b).next().unwrap())
        .map(priority)
        .sum::<u32>()
}

fn part2() -> u32 {
    stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            *chunk[0]
                .intersection(&chunk[1])
                .map(|c| c.to_owned())
                .collect::<HashSet<char>>()
                .intersection(&chunk[2])
                .next()
                .unwrap()
        })
        .map(priority)
        .sum::<u32>()
}

pub fn run() {
    const PART_1: bool = false;

    println!("{}", if PART_1 { part1() } else { part2() });
}
