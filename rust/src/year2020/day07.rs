use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

use regex::Regex;

#[derive(Eq, Hash, Debug)]
struct Bag {
    desc: String,
    color: String,
    // count: usize
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.desc == other.desc && self.color == other.color
    }
}

fn get_containing<'a>(
    bag: &'a Bag,
    contained_by: &'a HashMap<Bag, Vec<Bag>>,
    mut containing: &mut HashSet<&'a Bag>,
) {
    containing.insert(&bag);

    let bag_containing = contained_by.get(&bag);

    if let Some(bag_containing) = bag_containing {
        bag_containing
            .iter()
            .for_each(|parent_bag| get_containing(parent_bag, &contained_by, &mut containing));
    }
}

pub fn run() {
    let target_bag: Bag = Bag {
        desc: "shiny".to_string(),
        color: "gold".to_string(),
        // count: 0
    };

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re_target_bag = Regex::new(r"(\w*) (\w*) bags").unwrap();
    let re_contents_bag = Regex::new(r"(\d) (\w*) (\w*) bags?").unwrap();

    let mut contained_by = HashMap::new();
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let main_segments: Vec<&str> = line.split(" contain ").collect();

            let target_raw = re_target_bag.captures(main_segments[0]).unwrap();
            let target = Bag {
                desc: target_raw[1].to_string(),
                color: target_raw[2].to_string(),
                // count: 0
            };

            re_contents_bag
                .captures_iter(main_segments[1])
                .for_each(|bag_raw| {
                    let bag = Bag {
                        // count: 0,
                        desc: bag_raw[2].to_string(),
                        color: bag_raw[3].to_string(),
                    };

                    contained_by.entry(bag).or_insert(vec![]).push(Bag {
                        desc: target.desc.clone(),
                        color: target.color.clone(),
                        // count: bag_raw[1].parse().unwrap_or(0)
                    });
                })
        });

    let mut containing = HashSet::new();
    get_containing(&target_bag, &contained_by, &mut containing);
    println!("{}", containing.len() - 1);
}
