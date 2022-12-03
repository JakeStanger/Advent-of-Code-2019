use std::collections::HashSet;
use std::io::{stdin, Read};

pub fn run() {
    let mut program_str = String::new();
    stdin().read_to_string(&mut program_str).unwrap();

    let program: Vec<&str> = program_str
        .split("\n")
        .filter(|line| line.len() > 0)
        .collect();

    let jumps: Vec<usize> = program
        .iter()
        .enumerate()
        .filter_map(|e| {
            if (*e.1).contains("jmp") {
                Some(e.0)
            } else {
                None
            }
        })
        .collect();

    let acc: Vec<i32> = jumps
        .into_iter()
        .filter_map(|jump| {
            let mut ptr: i32 = 0;
            let mut acc: i32 = 0;

            let mut visited = HashSet::new();
            let terminates = loop {
                if visited.contains(&ptr) {
                    break false;
                } else {
                    visited.insert(ptr);
                }

                let instr_raw = program.get(ptr as usize);

                if let Some(instr_raw) = instr_raw {
                    let instr: Vec<&str> = instr_raw.split(" ").collect();
                    let op = if ptr as usize != jump {
                        instr[0]
                    } else {
                        "nop"
                    };
                    let val = instr[1].parse::<i32>().unwrap();

                    match op {
                        "acc" => {
                            acc += val;
                            ptr += 1;
                        }
                        "jmp" => ptr += val,
                        "nop" => ptr += 1,
                        _ => panic!(),
                    }
                } else {
                    break true;
                }
            };

            if terminates {
                Some(acc)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>();

    println!("{}", acc.first().unwrap());
}
