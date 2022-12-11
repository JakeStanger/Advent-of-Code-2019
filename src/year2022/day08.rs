use std::io::{stdin, BufRead};

fn slice_visibility(row: &[u32], height: u32) -> usize {
    row.iter().position(|h| *h >= height).unwrap_or(row.len()-1) + 1
}

// fn tallest_in_slice(row: &[u32], height: u32) -> bool {
//     slice_visibility(row, height) == 0
// }

// fn visible_vertical(trees: &[Vec<u32>], row: usize, col_n: usize, height: u32) -> bool {
//     let col = trees.iter().map(|row| row[col_n]).collect::<Vec<_>>();
//
//     tallest_in_slice(&col[..row], height) || tallest_in_slice(&col[row + 1..], height)
// }

fn scenic_score(trees: &[Vec<u32>], row: usize, col: usize, height: u32) -> usize {
    let trees_col = trees.iter().map(|row| row[col]).collect::<Vec<_>>();

    let mut left = trees[row][..col].to_vec();
    left.reverse();

    let mut top = trees_col[..row].to_vec();
    top.reverse();

    slice_visibility(&left, height)
        * slice_visibility(&trees[row][col + 1..], height)
        * slice_visibility(&top, height)
        * slice_visibility(&trees_col[row + 1..], height)
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
            row.iter()
                .enumerate()
                // .filter(|(col, height)| {
                //     let res = tallest_in_slice(&row[..*col], **height)
                //         || tallest_in_slice(&row[col + 1..], **height)
                //         || visible_vertical(&trees, row_n, *col, **height);
                //
                //     print!("{}", if res { 'x' } else { ' ' });
                //
                //     res
                // })
                // .count()
                .map(|(col, height)| scenic_score(&trees, row_n, col, *height))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("\n{res}");
}
