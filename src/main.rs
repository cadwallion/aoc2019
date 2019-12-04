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

    fn compute(&mut self, noun: usize, verb: usize) -> usize {
        self.codes[1] = noun;
        self.codes[2] = verb;
        loop {
            match self.codes[self.position] {
                1 => self.add(
                    self.codes[self.position + 1],
                    self.codes[self.position + 2],
                    self.codes[self.position + 3],
                ),
                2 => self.multiply(
                    self.codes[self.position + 1],
                    self.codes[self.position + 2],
                    self.codes[self.position + 3],
                ),
                99 => break,
                _ => panic!("Unknown opcode!"),
            }
            self.position += 4;
        }
        self.codes[0]
    }
}

fn day2a() {
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
    let result = computer.compute(12, 2);
    println!("Day2a: {}", result);
}

fn day2b() {
    let data = fs::read_to_string("./inputs/day2.txt").expect("Cannot read");
    let output = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let codes: Vec<usize> = data
                .trim()
                .split(",")
                .map(|code| code.parse::<usize>().unwrap())
                .collect();
            let mut computer = IntcodeComputer {
                codes: codes,
                position: 0,
            };
            if computer.compute(noun, verb) == output {
                println!("Day2b: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn main() {
    day1a();
    day1b();
    day2a();
    day2b();
}
