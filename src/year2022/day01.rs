use itertools::Itertools;
use std::io::{stdin, BufRead};

pub fn run() {
    let res: u32 = stdin()
        .lock()
        .lines()
        .map(|part| part.unwrap())
        .join("\n")
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|c| c.parse::<u32>().unwrap()))
        .map(|elf| elf.sum::<u32>())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    println!("{res:?}");
}
