use std::collections::HashSet;
use std::io::{stdin, BufRead};

pub fn run() {
    const MESSAGE_LENGTH: usize = 14;

    let line = stdin().lock().lines().next().unwrap().unwrap();

    let res = (MESSAGE_LENGTH..line.len())
        .find(|i| {
            line[*i - MESSAGE_LENGTH..*i]
                .chars()
                .collect::<HashSet<_>>()
                .len()
                == MESSAGE_LENGTH
        })
        .unwrap();

    println!("{res:?}");
}
