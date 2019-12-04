use std::fs::File;
use std::io::{BufRead, BufReader};

fn day1a() {
    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0.0;
    for (_, line) in reader.lines().enumerate() {
        sum += get_fuel_cost(line.unwrap().parse::<f64>().unwrap());
    }
    println!("Day 1a: {}", sum);
}

fn get_fuel_cost(mass: f64) -> f64 {
    ((mass / 3.0).floor()) - 2.0
}

// 4841054
fn day1b() {
    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0.0;
    for (_, line) in reader.lines().enumerate() {
        let mut mass = line.unwrap().parse::<f64>().unwrap();
        mass = get_fuel_cost(mass);
        while mass > 0.0 {
            sum += mass;
            mass = get_fuel_cost(mass);
        }
    }
    println!("Day 1b: {}", sum);
}

fn main() {
    day1a();
    day1b();
}
