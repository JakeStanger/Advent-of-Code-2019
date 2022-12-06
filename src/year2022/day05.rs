use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Inventory {
    inventory: HashMap<usize, VecDeque<char>>,
}

impl From<&str> for Inventory {
    fn from(input: &str) -> Self {
        let mut inventory = HashMap::new();

        /// `'[X] '.len()`
        const CHUNK_SIZE: usize = 4;

        let mut lines = input.split('\n').peekable();

        while let Some(line) = lines.next() {
            // ignore last line
            if lines.peek().is_some() {
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(CHUNK_SIZE)
                    .enumerate()
                    .for_each(|(j, chunk)| {
                        let char = *chunk.iter().nth(1).unwrap();
                        if char != ' ' {
                            inventory
                                .entry(j + 1)
                                .or_insert_with(VecDeque::new)
                                .push_back(char)
                        }
                    });
            }
        }

        Self { inventory }
    }
}

impl Inventory {
    fn move_items(&mut self, instr: Instruction) {
        for _ in 0..instr.count {
            let start_stack = self.inventory.get_mut(&instr.start).unwrap();
            let item = start_stack.pop_front().unwrap();

            let end_stack = self.inventory.get_mut(&instr.dest).unwrap();
            end_stack.push_front(item);
        }
    }

    fn get_code(&self) -> String {
        self.inventory.keys().sorted().map(|key| self.inventory.get(key).unwrap().get(0).unwrap()).collect()
    }
}

#[derive(Debug)]
struct Instruction {
    start: usize,
    dest: usize,
    count: usize,
}

impl From<&str> for Instruction {
    fn from(instr: &str) -> Self {
        // move {count} from {start} to {dest}
        let mut parts = instr.split(' ');

        let count = parts.nth(1).unwrap().parse().unwrap();
        let start = parts.nth(1).unwrap().parse().unwrap();
        let dest = parts.nth(1).unwrap().parse().unwrap();

        Self { count, start, dest }
    }
}

pub fn run() {
    let (mut inv, instructions) = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .join("\n")
        .split_once("\n\n")
        .map(|(a, b)| {
            (
                Inventory::from(a),
                b.split('\n').map(Instruction::from).collect::<Vec<_>>(),
            )
        })
        .unwrap();

    for instr in instructions {
        inv.move_items(instr);
    }

    println!("{}", inv.get_code())
}
