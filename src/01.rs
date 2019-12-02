use std::io::{stdin, BufRead};

fn calc_fuel(mass: usize) -> usize {
    (mass / 3) - 2
}

fn fuel_fuel(fuel: usize) -> usize {
    if (fuel / 3) > 1 {
        fuel_fuel(calc_fuel(fuel)) + fuel
    }
    else {
        fuel
    }
}

fn main() {
    let stdin = stdin();

    let mut total = 0;

    for line in stdin.lock().lines() {
        let mass = line.unwrap().parse().unwrap();
        let fuel = calc_fuel(mass);

        let extra_fuel = fuel_fuel(fuel);
        if extra_fuel != fuel {
            total += fuel_fuel(fuel);
        }
    }

    println!("{}", total);
}
