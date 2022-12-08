use std::io::{stdin, BufRead};

fn tallest_in_slice(row: &[u32], height: u32) -> bool {
    row.iter().filter(|h| **h >= height).count() == 0
}

fn visible_vertical(trees: &[Vec<u32>], row: usize, col_n: usize, height: u32) -> bool {
    let col = trees.iter().map(|row| row[col_n]).collect::<Vec<_>>();

    tallest_in_slice(&col[..row], height) || tallest_in_slice(&col[row+1..], height)
}

pub fn run() {
    let trees = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res = trees
        .iter()
        .enumerate()
        .map(|(row_n, row)| {
            print!("\n");
            row.iter()
                .enumerate()
                .filter(|(col, height)| {
                    let res = tallest_in_slice(&row[..*col], **height)
                        || tallest_in_slice(&row[col + 1..], **height)
                        || visible_vertical(&trees, row_n, *col, **height);

                    print!("{}", if res { 'x' } else { ' ' });

                    res
                })
                .count()
        })
        .sum::<usize>();

    println!("\n{res}");
}
