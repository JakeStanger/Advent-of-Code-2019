use std::io::{stdin, BufRead};

const GOAL: i32 = 2020;

pub fn run() {
    let stdin = stdin();
    let nums: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            let value = line.unwrap().parse().unwrap();
            if value < GOAL {
                Some(value)
            } else {
                None
            }
        })
        .collect();

    for num in &nums {
        for num2 in &nums {
            if num + num2 >= GOAL {
                continue;
            }

            let partner = nums.iter().find(|&num3| num + num2 + num3 == GOAL);
            if let Some(num3) = partner {
                println!("{}", num3 * num * num2);
                return;
            }
        }
    }
}
