use std::io::{stdin, Read};

const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_INP: i32 = 3;
const OP_OUT: i32 = 4;
const OP_TRU: i32 = 5;
const OP_FAL: i32 = 6;
const OP_LTN: i32 = 7;
const OP_EQU: i32 = 8;

const OP_HLT: i32 = 99;

const MOD_POS: i32 = 0;
const MOD_IMM: i32 = 1;

fn get_mode(instruction: i32) -> (i32, i32, i32) {
    (
        instruction / 100 % 10,
        instruction / 1000 % 10,
        instruction / 10000 % 10,
    )
}

fn get_operation(instruction: i32) -> i32 {
    instruction % 100
}

fn load(mode: i32, addr: i32, commands: &Vec<i32>) -> i32 {
    match mode {
        MOD_IMM => addr,
        MOD_POS => commands[addr as usize],
        _ => unreachable!(),
    }
}

fn store(addr: usize, value: i32, commands: &mut Vec<i32>) {
    commands[addr] = value;
}

fn run_computer(mut commands: Vec<i32>, input: (i32, i32)) -> Option<i32> {
    let mut ptr: usize = 0;
    loop {
        let instruction = commands[ptr];

        let opcode = get_operation(instruction);
        let mode = get_mode(instruction);

        match opcode {
            OP_ADD => {
                let op1 = load(mode.0, commands[ptr + 1], &commands);
                let op2 = load(mode.1, commands[ptr + 2], &commands);
                let dest = load(MOD_IMM, commands[ptr + 3], &commands);

                store(dest as usize, op1 + op2, &mut commands);

                ptr += 4;
            }
            OP_MUL => {
                let op1 = load(mode.0, commands[ptr + 1], &commands);
                let op2 = load(mode.1, commands[ptr + 2], &commands);
                let dest = load(MOD_IMM, commands[ptr + 3], &commands);

                store(dest as usize, op1 * op2, &mut commands);

                ptr += 4;
            }
            OP_INP => {
                let input = if ptr == 0 { input.0 } else { input.1 };

                let addr = load(MOD_IMM, commands[ptr + 1], &commands);
                store(addr as usize, input, &mut commands);

                ptr += 2;
            }
            OP_OUT => {
                let val = load(mode.0, commands[ptr + 1], &commands);

                println!("OUT\t{}", val);
                // ptr += 2;
                return Some(val);
            }
            OP_TRU => {
                let jump = load(mode.0, commands[ptr + 1], &commands);
                if jump != 0 {
                    let dest = load(mode.1, commands[ptr + 2], &commands);
                    ptr = dest as usize;
                } else {
                    ptr += 3;
                }
            }
            OP_FAL => {
                let jump = load(mode.0, commands[ptr + 1], &commands);
                if jump == 0 {
                    let dest = load(mode.1, commands[ptr + 2], &commands);
                    ptr = dest as usize;
                } else {
                    ptr += 3;
                }
            }
            OP_LTN => {
                let op1 = load(mode.0, commands[ptr + 1], &commands);
                let op2 = load(mode.1, commands[ptr + 2], &commands);
                let dest = load(MOD_IMM, commands[ptr + 3], &commands);

                store(dest as usize, (op1 < op2) as i32, &mut commands);

                ptr += 4;
            }
            OP_EQU => {
                let op1 = load(mode.0, commands[ptr + 1], &commands);
                let op2 = load(mode.1, commands[ptr + 2], &commands);
                let dest = load(MOD_IMM, commands[ptr + 3], &commands);

                store(dest as usize, (op1 == op2) as i32, &mut commands);

                ptr += 4;
            }
            OP_HLT => return None,
            _ => unreachable!(opcode),
        }
    }
}

pub fn run() {
    let mut cmd_string = String::new();
    stdin().read_to_string(&mut cmd_string).unwrap();
    let commands: Vec<i32> = cmd_string
        .trim()
        .split(",")
        .map(|cmd| cmd.parse().unwrap())
        .collect();

    let mut max = 0;
    for a in 5..=9 {
        for b in (5..=9).filter(|&b| a != b) {
            for c in (5..=9).filter(|&c| a != c && b != c) {
                for d in (5..=9).filter(|&d| a != d && b != d && c != d) {
                    for e in (5..=9).filter(|&e| a != e && b != e && c != e && d != e) {
                        let seq = [a, b, c, d, e];
                        max = max.max((0..5).fold(0, |acc, n| {
                            run_computer(commands.clone(), (seq[n], acc)).unwrap()
                        }))
                    }
                }
            }
        }
    }

    println!("{}", max);
}
