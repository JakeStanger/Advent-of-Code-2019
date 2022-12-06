use std::collections::HashSet;
use std::io::{stdin, BufRead};

pub fn run() {
    let line = stdin().lock().lines().next().unwrap().unwrap();

    let res = (4..line.len())
        .find(|i| line[*i - 4..*i].chars().collect::<HashSet<_>>().len() == 4)
        .unwrap();

    println!("{res:?}");
}
