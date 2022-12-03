use std::io::{stdin, BufRead};

const TREE: char = '#';

fn trees_for_slope(map: &Vec<String>, route: &(usize, usize)) -> u64 {
    let mut position = (0, 0);

    let height = map.len() - 1;
    let width = map[0].len() - 1;

    let mut trees: u64 = 0;
    while position.0 < height {
        if map[position.0].as_bytes()[position.1] == TREE as u8 {
            trees += 1;
        }

        position = (position.0 + route.0, {
            let new_col = position.1 + route.1;
            if new_col <= width {
                new_col
            } else {
                new_col - (width + 1)
            }
        });
    }

    trees
}

pub fn run() {
    let stdin = stdin();
    let map = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let product = slopes
        .iter()
        .map(|slope| trees_for_slope(&map, slope))
        .fold(1, |prod, trees| prod * trees);

    println!("{}", product);
}
