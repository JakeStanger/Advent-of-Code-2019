use std::io::{stdin, BufRead};
use std::collections::{HashMap};

fn get_orbiting(moon: &String, mut total: i64, pairs: &HashMap<String, String>) -> i64 {
    if moon == "COM" {
        return total
    }

    let parent = pairs.get(moon).unwrap();

    total += 1;
    get_orbiting(parent, total, &pairs)
}

fn main() {
    let stdin = stdin();

    let mut pairs: HashMap<String, String> = HashMap::new();

    stdin.lock().lines()
    .map(|line| line.unwrap())
    .for_each(|line| {
        let mut bodies = line.split(')');
        let parent = bodies.next().unwrap().to_string();
        let moon = bodies.next().unwrap().to_string();

        pairs.insert(moon, parent);
    });

    let mut total: i64 = 0;
    for moon in pairs.keys() {
        total += get_orbiting(moon, 0, &pairs);
    }

    println!("{}", total);

    
}