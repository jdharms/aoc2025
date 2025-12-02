use std::{env, str::FromStr};

use aoc2025::{get_file_name, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile = get_file_name(&args).unwrap();

    let lines = read_lines(infile);
    let contents = lines[0].split(',');
    let ranges: Vec<Range> = contents.map(|s| s.parse().unwrap()).collect();

    let answer = ranges.iter().fold(0, |acc, r| {
        acc + r.get_invalid(first_validator).iter().fold(0, |acc, v| { acc + v })
    });

    let answer_two = ranges.iter().fold(0, |acc, r| {
        acc + r.get_invalid(second_validator).iter().fold(0, |acc, v| { acc + v })
    });

    println!("The answer is {answer}");
    println!("Answer two is {answer_two}");
}

fn first_validator(i: &u64) -> bool {
    let s = i.to_string();
    if s.len() % 2 != 0 {
        return false;
    }

    let (first, second) = s.split_at(s.len()/2);
    first == second
}



fn second_validator(i: &u64) -> bool {
    let s = i.to_string();

    for j in (Factors{number: s.len()}) {
        // Are the first j characters of s repeated (s.len() / j) - 1 more times?
        let (first, rest) = s.split_at(j);
        if rest.matches(first).count() == (s.len() / j) - 1 {
            return true
        }
    }


    false
}

struct FactorIter {
    curr: usize,
    number: usize,
}

impl Iterator for FactorIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        let max = self.number;

        for i in current+1..max {
            if max % i == 0 {
                self.curr = i;
                return Some(i);
            }
        }

        None
    }
}

struct Factors {
    number: usize,
}

impl IntoIterator for Factors {
    type Item = usize;

    type IntoIter = FactorIter;

    fn into_iter(self) -> Self::IntoIter {
        FactorIter{curr: 0, number: self.number}
    }
}

struct Range {
    start: u64,
    end: u64,
}

type Validator = fn(&u64) -> bool;

impl Range {
    pub fn get_invalid(&self, validator: Validator) -> Vec<u64> {
        (self.start..(self.end+1)).filter(validator).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((start_str, end_str)) => {
                let start: u64 = start_str.parse().map_err(|_| ParseRangeError)?;
                let end: u64 = end_str.parse().map_err(|_| ParseRangeError)?;
                Ok(Range{start, end})
            },
            None => Ok(Range{start:0,end:0}),
        }
    }
}