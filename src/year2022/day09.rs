use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    dir: Direction,
    count: i32,
}

impl From<String> for Movement {
    fn from(str: String) -> Self {
        let (dir, count) = str.split_once(' ').unwrap();
        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };

        Self {
            dir,
            count: count.parse().unwrap(),
        }
    }
}

type Pos = (i32, i32);

struct State {
    head: Pos,
    tail: Pos,
    tail_history: HashSet<Pos>,
}

impl State {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            tail_history: HashSet::new(),
        }
    }

    fn move_head(&mut self, movement: Movement) {
        // println!("=== {:?} {} === ", movement.dir, movement.count);

        // println!("{self}");

        for _ in 0..movement.count {
            self.head = match movement.dir {
                Direction::Up => (self.head.0, self.head.1 + 1),
                Direction::Left => (self.head.0 - 1, self.head.1),
                Direction::Down => (self.head.0, self.head.1 - 1),
                Direction::Right => (self.head.0 + 1, self.head.1),
            };

            if (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1 {
                self.move_tail(movement.dir)
            }

            // println!("{}", head_tail_distance);
            // println!("{self}");

            //     println!(
            //     "H: {:?} | T: {:?} | V: {}",
            //     self.head,
            //     self.tail,
            //     self.tail_history.len()
            // );
        }
    }

    fn move_tail(&mut self, dir: Direction) {
        self.tail = match dir {
            Direction::Up => (self.head.0, self.head.1 - 1),
            Direction::Left => (self.head.0 + 1, self.head.1),
            Direction::Down => (self.head.0, self.head.1 + 1),
            Direction::Right => (self.head.0 - 1, self.head.1),
        };
        self.tail_history.insert(self.tail);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "H: {:?} | T: {:?} | V: {}\n\n",
            self.head,
            self.tail,
            self.tail_history.len()
        )?;
        for y in (0..5).rev() {
            for x in 0..6 {
                write!(
                    f,
                    "{}",
                    if (x, y) == self.head {
                        'H'
                    } else if (x, y) == self.tail {
                        'T'
                    } else {
                        '.'
                    }
                )?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub fn run() {
    let mut state = State::new();

    stdin()
        .lock()
        .lines()
        .map(|line| Movement::from(line.unwrap()))
        .for_each(|movement| state.move_head(movement));

    println!("{}", state.tail_history.len() + 1)
}
