use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

use regex::Regex;

#[derive(Eq, Hash, Debug)]
struct Bag {
    desc: String,
    color: String,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.desc == other.desc && self.color == other.color
    }
}

struct Count(usize);

fn get_containing(
    bag: &(Bag, usize),
    containing: &HashMap<Bag, Vec<(Bag, usize)>>,
    // count: &mut Count,
) -> usize {
    let bag_containers = containing.get(&bag.0);

    if let Some(bag_containers) = bag_containers {
        bag_containers
            .iter()
            .map(|child_bag| child_bag.1 + child_bag.1 * get_containing(&child_bag, &containing))
            .sum()
    } else {
        0
    }
}

pub fn run() {
    let target_bag: Bag = Bag {
        desc: "shiny".to_string(),
        color: "gold".to_string(),
    };

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re_target_bag = Regex::new(r"(\w*) (\w*) bags").unwrap();
    let re_contents_bag = Regex::new(r"(\d) (\w*) (\w*) bags?").unwrap();

    let mut containing = HashMap::new();

    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let main_segments: Vec<&str> = line.split(" contain ").collect();

            let target_raw = re_target_bag.captures(main_segments[0]).unwrap();
            let target = Bag {
                desc: target_raw[1].to_string(),
                color: target_raw[2].to_string(),
            };

            re_contents_bag
                .captures_iter(main_segments[1])
                .for_each(|bag_raw| {
                    let bag = Bag {
                        desc: bag_raw[2].to_string(),
                        color: bag_raw[3].to_string(),
                    };

                    containing
                        .entry(Bag {
                            desc: target.desc.clone(),
                            color: target.color.clone(),
                        })
                        .or_insert(vec![])
                        .push((bag, bag_raw[1].parse::<usize>().unwrap_or(0)));
                })
        });

    let count = get_containing(&(target_bag, 0), &containing);

    println!("{}", count);
}
