use std::fs;

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
pub fn a() {
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
