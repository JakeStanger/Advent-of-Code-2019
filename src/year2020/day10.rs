use std::io::{stdin, BufRead};

pub fn run() {
    let stdin = stdin();
    let ratings: Vec<u32> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let max_rating = ratings.iter().max().unwrap();
    let device_rating = max_rating + 3;

    println!("{}", device_rating);
}
