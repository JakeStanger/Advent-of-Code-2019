use std::cmp::max;
use std::io::{stdin, BufRead};
use itertools::Itertools;

pub fn run() {
    let res: u32 = stdin()
        .lock()
        .lines()
        .map(|part| part.unwrap())
        .enumerate()
        .fold(vec![vec![]], |mut acc, (i, n)| {
            if !n.is_empty() {
                acc.last_mut().unwrap().push(n.parse::<u32>().unwrap());
            } else {
                acc.push(vec![]);
            }
            acc
        })
        .into_iter()
        .map(|vec| vec.into_iter().sum())
        .sorted_by(|a: &u32, b: &u32| b.cmp(a))
        .take(3)
        .sum();

    println!("{res:?}");
}
