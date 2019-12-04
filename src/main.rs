use std::fs::File;
use std::io::{BufRead, BufReader};

fn day1() {
    let file = File::open("./inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0.0;
    for (_, line) in reader.lines().enumerate() {
        let mass: f64 = line.unwrap().parse::<f64>().unwrap();
        let value = ((mass / 3.0).floor()) - 2.0;
        sum += value;
    }
    println!("The sum is {}", sum);
}

fn main() {
    day1();
}
