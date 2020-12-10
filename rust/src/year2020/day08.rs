use std::io::{stdin, Read};
use std::collections::HashSet;

pub fn run() {
    let mut ptr: i32 = 0;
    let mut acc: i32 = 0;

    let mut program_str = String::new();
    stdin().read_to_string(&mut program_str).unwrap();

    let program: Vec<&str> = program_str.split("\n").collect();

    let mut visited = HashSet::new();
    loop {
        if visited.contains(&ptr) {
            break;
        } else {
            visited.insert(ptr);
        }

        let instr: Vec<&str> = program[ptr as usize].split(" ").collect();
        let op = instr[0];
        let val = instr[1].parse::<i32>().unwrap();

        match op {
            "acc" => {
                acc += val;
                ptr += 1;
            }
            "jmp" => ptr += val,
            "nop" => ptr += 1,
            _ => panic!()
        }
    }

    println!("{}", acc);
}
