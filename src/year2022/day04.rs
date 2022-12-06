use std::io::{stdin, BufRead};

fn to_range(str: &str) -> (u32, u32) {
    str.split_once('-')
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .unwrap()
}

// fn engulfs((a1, a2): (u32, u32), (b1, b2): (u32, u32)) -> bool {
//     a1 <= b1 && a2 >= b2
// }

fn overlaps((a1, a2): (u32, u32), (b1, b2): (u32, u32)) -> bool {
    a1 <= b2 && b1 <= a2
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
        // .filter(|(a, b)| engulfs(*a, *b) || engulfs(*b, *a))
        .filter(|(a, b)| overlaps(*a, *b) && overlaps(*b, *a))
        .count();

    println!("{res:?}");
}
