use std::collections::HashSet;
use std::io::{stdin, BufRead};

pub fn run() {
    let stdin = stdin();
    let ids: HashSet<i32> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|pass| {
            let mut min_row = 0;
            let mut max_row = 127;

            let mut min_col = 0;
            let mut max_col = 7;

            pass.chars().for_each(|char| {
                let row_range = max_row - min_row;
                let col_range = max_col - min_col;

                match char {
                    'F' => max_row = max_row - ((row_range / 2) + 1),
                    'B' => min_row = min_row + (row_range / 2) + 1,
                    'L' => max_col = max_col - ((col_range / 2) + 1),
                    'R' => min_col = min_col + (col_range / 2) + 1,
                    _ => {}
                };
            });

            // these are the same at this point
            let row = min_row;
            let col = min_col;

            let id = row * 8 + col;

            id
        })
        .collect();

    let id = ids
        .iter()
        .find(|&id| !ids.contains(&(id + 1)) && ids.contains(&(id + 2)))
        .unwrap()
        + 1;

    println!("{:?}", id);
}
