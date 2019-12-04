use std::fs;
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

struct IntcodeComputer {
    codes: Vec<usize>,
    position: usize,
}

impl IntcodeComputer {
    fn multiply(&mut self, factor1_pos: usize, factor2_pos: usize, product_pos: usize) {
        self.codes[product_pos] = self.codes[factor1_pos] * self.codes[factor2_pos];
    }

    fn add(&mut self, addend1_pos: usize, addend2_pos: usize, sum_pos: usize) {
        self.codes[sum_pos] = self.codes[addend1_pos] + self.codes[addend2_pos];
    }
}

// opcodes:
// 99 - exit
// 1 - add two numbers from the positions in index 1/2, store at position in index 3
// 2 - multiply two numbers from the positions in index 1/2, store at position in index 3
// when complete with an opcode, advance 4 positions
fn day2() {
    //let file = File::open("./inputs/day2.txt").unwrap();
    //let reader = BufReader::new(file);
    let data = fs::read_to_string("./inputs/day2.txt").expect("Cannot read");
    let codes: Vec<usize> = data
        .trim()
        .split(",")
        .map(|code| code.parse::<usize>().unwrap())
        .collect();
    let mut computer = IntcodeComputer {
        codes: codes,
        position: 0,
    };
    loop {
        match computer.codes[computer.position] {
            1 => computer.add(
                computer.codes[computer.position + 1],
                computer.codes[computer.position + 2],
                computer.codes[computer.position + 3],
            ),
            2 => computer.multiply(
                computer.codes[computer.position + 1],
                computer.codes[computer.position + 2],
                computer.codes[computer.position + 3],
            ),
            99 => break,
            _ => panic!("Unknown opcode!"),
        }
        computer.position += 4;
    }
    println!("Day2a: {}", computer.codes[0]);
}

fn main() {
    day1a();
    day1b();
    day2();
}
