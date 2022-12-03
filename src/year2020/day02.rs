use regex::Regex;
use std::io::{stdin, BufRead, Error};

pub fn run() {
    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}) ([a-z]): ([a-z]+)$").unwrap();

    let stdin = stdin();
    let num_valid: usize = stdin
        .lock()
        .lines()
        .filter(|line| {
            let raw = line.as_ref().unwrap().as_str();
            let captures = re.captures(raw).unwrap();

            let min = &captures[1].parse::<usize>().unwrap();
            let max = &captures[2].parse::<usize>().unwrap();
            let letter = captures[3].parse::<char>().unwrap();
            let password = &captures[4];

            // part 1
            // let count = password.matches(letter).count();
            // &count >= min && &count <= max

            // part 2
            let pos1 = min - 1;
            let pos2 = max - 1;

            let char1 = password.as_bytes()[pos1];
            let char2 = password.as_bytes()[pos2];

            (char1 == letter as u8) ^ (char2 == letter as u8)
        })
        .collect::<Vec<Result<String, Error>>>()
        .len();

    println!("{}", num_valid);
}
