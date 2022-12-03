use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn get_orbits(moon: &String, mut total: i32, pairs: &HashMap<String, String>, end: String) -> i32 {
    if *moon == end {
        return total;
    }

    let parent = pairs.get(moon).unwrap();

    total += 1;
    get_orbits(parent, total, &pairs, end)
}

fn get_orbiting(
    moon: &String,
    mut path: Vec<String>,
    pairs: &HashMap<String, String>,
) -> Vec<String> {
    if moon == "COM" {
        return path;
    }

    let parent = pairs.get(moon).unwrap();
    path.push(parent.to_string());

    get_orbiting(parent, path, &pairs)
}

pub fn run() {
    let stdin = stdin();

    let mut pairs: HashMap<String, String> = HashMap::new();

    stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let mut bodies = line.split(')');
            let parent = bodies.next().unwrap().to_string();
            let moon = bodies.next().unwrap().to_string();

            pairs.insert(moon, parent);
        });

    let mut total: i32 = 0;
    for moon in pairs.keys() {
        total += get_orbits(moon, 0, &pairs, String::from("COM"));
    }

    let you = pairs.get("YOU").unwrap();
    let santa = pairs.get("SAN").unwrap();

    let you_path = get_orbiting(you, Vec::new(), &pairs);
    let santa_path = get_orbiting(santa, Vec::new(), &pairs);

    let intersection = you_path.iter().filter(|moon| santa_path.contains(moon));

    let mut shortest = i32::max_value();
    for moon in intersection {
        let you_dist = get_orbits(you, 0, &pairs, moon.to_string());
        let santa_dist = get_orbits(santa, 0, &pairs, moon.to_string());

        let total = you_dist + santa_dist;

        if total < shortest {
            shortest = total;
        }
    }

    println!("p1: {}", total);
    println!("p2: {}", shortest);
}
