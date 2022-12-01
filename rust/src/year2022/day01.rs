use itertools::Itertools;
use std::cmp::max;
use std::io::{stdin, BufRead};

pub fn run() {
    let res: u32 = stdin()
        .lock()
        .lines()
        .map(|part| part.unwrap())
        .join("\n")
        .split("\n\n")
        .map(|lines| lines.split('\n').map(|n| n.parse::<u32>().unwrap()))
        .map(|vec| vec.sum())
        .sorted_by(|a: &u32, b: &u32| b.cmp(a))
        .take(3)
        .sum();

    println!("{res:?}");
}
