mod year2019;
mod year2020;

use std::env;

fn main() {
    let year: u32 = env::var("YEAR").unwrap().parse().unwrap();
    let day: u32 = env::var("DAY").unwrap().parse().unwrap();

    match (year, day) {
        // 2019
        (2019, 1) => year2019::day01::run(),

        (2020, 1) => year2020::day01::run(),
        (_, _) => panic!("invalid year/day"),
    }
}
