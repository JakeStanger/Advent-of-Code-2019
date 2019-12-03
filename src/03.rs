use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn get_between(dir: char, distance: &i32, current: (i32, i32)) -> Vec<(i32, i32)> {
    match dir {
        'U' => (1..=*distance).map(|i| (current.0, current.1 + i)).collect(),
        'D' => (1..=*distance).map(|i| (current.0, current.1 - i)).collect(),
        'L' => (1..=*distance).map(|i| (current.0 - i, current.1)).collect(),
        'R' => (1..=*distance).map(|i| (current.0 + i, current.1)).collect(),
        _ => panic!()
    }
}

fn main() {
    let stdin = stdin();

    let lines = stdin.lock().lines();

    let mut all_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut shortest = i32::max_value();

    let mut line_num = 0;
    for line in lines {
        let line_value = line.unwrap();
        let instructions = line_value.split(",");

        let mut visited: Vec<(i32, i32)> = vec![(0, 0)];  

        for instruction in instructions {
            let direction = instruction.chars().next().unwrap();
            let distance: &i32 = &instruction[1..instruction.len()].parse::<i32>().unwrap();

            let last_visited = visited[visited.len()-1];

            let mut current_visited: Vec<(i32, i32)> = get_between(direction, distance, last_visited);
            
            visited.append(&mut current_visited.clone());

            for point in current_visited {
                if !all_visited.contains(&point) {
                    all_visited.insert(point);
                }
                else if line_num == 1 { 
                    let dist = point.0.abs() + point.1.abs();
                    if dist < shortest && dist != 0 {
                        shortest = dist;
                    }
                }
            }
        }

        line_num += 1;
    }

    println!("{}", shortest);
}