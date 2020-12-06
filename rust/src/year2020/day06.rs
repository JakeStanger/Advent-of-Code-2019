use std::collections::HashMap;
use std::io::{stdin, Read};

pub fn run() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let total = input
        .split("\n\n")
        .map(|group| {
            let people: Vec<String> = group
                .split("\n")
                .map(|questions| questions.to_string())
                .filter(|questions| questions.len() > 0)
                .collect();
            let mut questions = HashMap::new();

            people.iter().for_each(|answers| {
                answers
                    .chars()
                    .for_each(|answer| *questions.entry(answer).or_insert(0) += 1)
            });

            let num_people = people.len();

            questions
                .into_iter()
                .filter(|&question| question.1 == num_people)
                .collect::<HashMap<char, usize>>()
                .len()
        })
        .fold(0, |sum, next| sum + next);

    println!("{}", total);
}
