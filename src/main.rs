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

type Point = (isize, isize);
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn crossing(vertical: &Segment, horizontal: &Segment) -> bool {
        vertical.start.0 >= horizontal.start.0
            && vertical.start.0 <= horizontal.end.0
            && ((vertical.start.1 >= horizontal.start.1 && vertical.end.1 <= horizontal.start.1)
                || (vertical.start.1 <= horizontal.start.1 && vertical.end.1 >= horizontal.start.1))
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
}

struct Wire {
    segments: Vec<Segment>,
    x: isize,
    y: isize,
}

impl Wire {
    fn move_y(&mut self, distance: &isize) {
        let new_y = self.y + distance;
        let segment = Segment {
            start: (self.x, self.y),
            end: (self.x, new_y),
        };
        self.y = new_y;
        self.segments.push(segment);
    }

    fn move_x(&mut self, distance: &isize) {
        let new_x = self.x + distance;
        let segment = Segment {
            start: (self.x, self.y),
            end: (new_x, self.y),
        };
        self.x = new_x;
        self.segments.push(segment);
    }

    fn from_paths(paths: &Vec<&str>) -> Wire {
        let mut wire = Wire {
            x: 0,
            y: 0,
            segments: Vec::<Segment>::new(),
        };
        for step in paths {
            let distance = &step[1..].parse::<isize>().unwrap();
            match &step[0..=0] {
                "U" => wire.move_y(distance),
                "D" => wire.move_y(&(0 - distance)),
                "L" => wire.move_x(&(0 - distance)),
                "R" => wire.move_x(distance),
                _ => panic!("Invalid instruction {}!", &step[0..=0]),
            }
        }
        wire
    }
}

// find collisions
//
// if going left/right, collision detection is only on the X axis
// if going up/down, collision detection is only on the Y axis
// calculate the x/y distance from origin to collision
fn day3a() {
    let data = fs::read_to_string("./inputs/day3.txt").expect("Cannot read");
    let wire_paths: Vec<Vec<&str>> = data
        .trim()
        .split("\n")
        .map(|path| path.split(",").collect())
        .collect();
    let wire1 = Wire::from_paths(&wire_paths[0]);
    let wire2 = Wire::from_paths(&wire_paths[1]);
    let mut crossings = Vec::<Point>::new();
    for segment1 in wire1.segments.iter() {
        for segment2 in wire2.segments.iter() {
            if segment1.is_vertical() && segment2.is_horizontal() {
                if Segment::crossing(segment1, segment2)
                    && segment1.start.1 != 0
                    && segment2.start.0 != 0
                {
                    crossings.push((segment1.start.0, segment2.start.1));
                }
            } else if segment2.is_vertical() && segment1.is_horizontal() {
                if Segment::crossing(segment2, segment1)
                    && segment2.start.1 != 0
                    && segment1.start.0 != 0
                {
                    crossings.push((segment2.start.0, segment1.start.1));
                }
            }
        }
    }
    let shortest = crossings
        .into_iter()
        .map(|point| point.0.abs() + point.1.abs())
        .fold(None, |min, distance| match min {
            None => Some(distance),
            Some(current) => Some(if distance < current {
                distance
            } else {
                current
            }),
        })
        .unwrap();

    println!("Day 3a: {}", shortest);
}

fn main() {
    day1a();
    day1b();
    day2a();
    day2b();
    day3a();
}
