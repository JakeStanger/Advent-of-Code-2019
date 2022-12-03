use std::io::{stdin, BufRead};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Shape {
    fn from(str: &str) -> Self {
        match str {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl From<&str> for Outcome {
    fn from(str: &str) -> Self {
        match str {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}

impl Shape {
    fn score(&self, other: &Self) -> u32 {
        let weapon_score = match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome_score = if self.beats(other) {
            6
        } else if self == other {
            3
        } else {
            0
        };

        weapon_score + outcome_score
    }

    fn beats(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Shape::Rock, Shape::Scissors)
                | (Shape::Paper, Shape::Rock)
                | (Shape::Scissors, Shape::Paper)
        )
    }

    fn pick_from_outcome(&self, outcome: Outcome) -> Self {
        if outcome != Outcome::Draw {
            match (self, outcome) {
                (Shape::Rock, Outcome::Win) | (Shape::Scissors, Outcome::Lose) => Shape::Paper,
                (Shape::Rock, Outcome::Lose) | (Shape::Paper, Outcome::Win) => Shape::Scissors,
                (Shape::Paper, Outcome::Lose) | (Shape::Scissors, Outcome::Win) => Shape::Rock,
                _ => unreachable!(),
            }
        } else {
            *self
        }
    }
}

pub fn run() {
    let res = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(' ')
                .map(|(a, b)| (Shape::from(a), Outcome::from(b)))
                .unwrap()
        })
        .map(|(a, b)| a.pick_from_outcome(b).score(&a))
        .sum::<u32>();

    println!("{res:?}")
}
