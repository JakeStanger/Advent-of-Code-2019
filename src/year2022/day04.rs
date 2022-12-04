use std::io::{stdin, BufRead};
use std::ops::Range;

fn to_range(str: &str) -> Range<u32> {
    let (min, max) = str.split_once('-').unwrap();
    Range {
        start: min.parse().unwrap(),
        end: max.parse().unwrap(),
    }
}

fn engulfs(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    range1.start <= range2.start && range1.end >= range2.end
}

fn overlaps(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    range1.start <= range2.end && range2.start <= range2.end
}

pub fn run() {
    let res = stdin()
        .lock()
        .lines()
        .map(|line| {
            Some(line.unwrap().split_once(',').unwrap())
                .map(|(a, b)| (to_range(a), to_range(b)))
                .unwrap()
        })
        // .filter(|(a, b)| engulfs(&a, &b) || engulfs(&b, &a))
        .filter(|(a, b)| overlaps(&a, &b) && overlaps(&b, &a))
        .count();

    println!("{res:?}");
}
