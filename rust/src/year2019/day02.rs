use std::io::{stdin, Read};
use std::process;

fn calc(noun: usize, verb: usize, mut commands: Vec<usize>) -> usize {
    commands[1] = noun;
    commands[2] = verb;

    let mut result = 0;
    for i in 0..commands.len()/4 {
        let pos = i * 4;
        let opcode = commands[pos];

        let pos1 = commands[pos+1];
        let pos2 = commands[pos+2];
        let dest = commands[pos+3];

        if opcode == 1 {
            commands[dest] = commands[pos1] + commands[pos2];
        }
        else if opcode == 2 {
            commands[dest] = commands[pos1] * commands[pos2];
        }
        else if opcode == 99 {
            result = commands[0]
        }
    }

    result
}

fn main() {
    let mut cmd_string = String::new();
    stdin().read_to_string(&mut cmd_string).unwrap();
    let commands: Vec<usize> = cmd_string.trim().split(",").map(|cmd| cmd.parse().unwrap()).collect();

    let desired_output = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let result = calc(noun, verb, commands.clone());
            if result == desired_output {
                println!("{}", 100 * noun + verb);
                process::exit(0);
            }
        }
    }
}