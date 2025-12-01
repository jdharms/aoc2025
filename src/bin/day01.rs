use std::env;
use std::str::FromStr;

use aoc2025::{get_file_name, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile = get_file_name(&args).unwrap();

    let contents = read_lines(infile);
    let rotations: Vec<Rotation> = contents.iter().map(|l| l.parse().unwrap()).collect();

    let mut dial = Dial::new();

    for r in rotations.iter() {
        dial.apply_rotation(r);
    }

    let times_zeroed = dial.times_zeroed;
    let total_zeroes = dial.zeroes_crossed;

    println!("The dial was equal to zero {times_zeroed} times after rotations");
    println!("The dial crossed zero {total_zeroes} times");
}


struct Dial {
    value: u8,
    zeroes_crossed: u32,
    times_zeroed: u32,
}

impl Dial {
    pub fn new() -> Self {
        Self { value: 50, zeroes_crossed: 0, times_zeroed: 0 }
    }

    pub fn apply_rotation(&mut self, rotation: &Rotation) {
        let (zeroes_crossed, new_value) = match rotation.direction {
            Direction::Left => (dial_left_zeroes(self.value, rotation.value), dial_subtract(self.value, rotation.value)),
            Direction::Right => (dial_right_zeroes(self.value, rotation.value), dial_add(self.value, rotation.value)),
        };
        self.zeroes_crossed += zeroes_crossed;
        self.value = new_value;
        if self.value == 0 {
            self.times_zeroed += 1;
        }
    }
}

fn dial_subtract(start: u8, turn: u32) -> u8 {
    // First insight is that you can just "ignore" rotations in multiples
    // of 100s.
    let turn = (turn % 100) as u8;

    // If we need to turn left more than we have "to go"
    if turn > start {
        100 - (turn - start)
    } else {
        start - turn
    }
}


fn dial_left_zeroes(start: u8, turn: u32) -> u32 {
    // Every 100 is exactly one click over the zero
    let mut res = turn / 100;

    if turn % 100 == 0 {
        return res;
    }

    // If we're turning left as far as we have "to go"
    // then we'll click over exactly one more time,
    // but only if we're not *starting* on zero.
    if turn % 100 >= start as u32 && start != 0 {
        res += 1;
    }

    res
}

fn dial_add(start: u8, turn: u32) -> u8 {
    let res = (start as u32 + turn) % 100;
    res as u8
}

fn dial_right_zeroes(start: u8, turn: u32) -> u32 {
    // every 100 is exactly one click over the zero
    let mut res = turn / 100;

    if turn % 100 == 0 {
        return res;
    }

    // if we're turning right at least (100 - start)
    // then we'll click over exactly one more time
    if turn % 100 >= (100 - start) as u32 {
        res += 1;
    } 

    res
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    value: u32,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRotationError;

impl FromStr for Rotation {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, mag) = s.split_at(1);

        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseRotationError),
        };

        let magnitude: u32 = match mag.parse() {
            Ok(number) => number,
            Err(_) => return Err(ParseRotationError),
        };

        Ok(Rotation {
            value: magnitude,
            direction: direction,
        })
    }
    
    type Err = ParseRotationError;
}