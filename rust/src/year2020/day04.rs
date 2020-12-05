use std::collections::HashMap;
use std::io::{stdin, Read};

use regex::Regex;

pub fn get_number_key(key: &str, data: &HashMap<String, String>) -> u32 {
    data.get(key).unwrap().parse::<u32>().unwrap()
}

pub fn run() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re_passport = Regex::new(r"(?m)([a-z]{3}):([^ \n]*)").unwrap();

    let re_height = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
    let re_hair = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();

    let required_keys: Vec<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|&key| key.to_string())
        .collect();

    let valid_eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let valid = input
        .split("\n\n")
        .filter(|&passport| {
            let mut data = HashMap::new();
            re_passport.captures_iter(passport).for_each(|cap| {
                data.insert(cap[1].to_string(), cap[2].to_string());
            });

            let keys = data.keys().collect::<Vec<_>>();
            let has_required_keys = required_keys.iter().all(|key| keys.contains(&key));

            if !has_required_keys {
                return false;
            }

            let birth_year = get_number_key("byr", &data);
            let valid_birth_year = birth_year >= 1920 && birth_year <= 2002;

            let issue_year = get_number_key("iyr", &data);
            let valid_issue_year = issue_year >= 2010 && issue_year <= 2020;

            let expiration_year = get_number_key("eyr", &data);
            let valid_expiration_year = expiration_year >= 2020 && expiration_year <= 2030;

            let raw_height = data.get("hgt").unwrap();
            let height_caps = re_height.captures(raw_height);

            let valid_height = if let Some(height_caps) = height_caps {
                let height = height_caps[1].parse::<u32>().unwrap();
                let height_unit = &height_caps[2];

                match height_unit {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                }
            } else {
                false
            };

            let hair_colour = data.get("hcl").unwrap();
            let valid_hair_colour = re_hair.captures(hair_colour).is_some();

            let eye_colour = data.get("ecl").unwrap();
            let valid_eye_colour = valid_eye_colours.iter().any(|&colour| colour == eye_colour);

            let pid = data.get("pid").unwrap();
            let valid_pid = re_pid.captures(pid).is_some();

            println!("{:?}", data);
            println!("{} {} {} {} {} {} {}\n\n", valid_birth_year, valid_issue_year, valid_expiration_year, valid_height, valid_hair_colour, valid_eye_colour, valid_pid);

            valid_birth_year
                && valid_issue_year
                && valid_expiration_year
                && valid_height
                && valid_hair_colour
                && valid_eye_colour
                && valid_pid
        })
        .collect::<Vec<&str>>()
        .len();

    println!("{}", valid);
}
