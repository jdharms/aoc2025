use std::{env, str::FromStr};

use aoc2025::{get_file_name, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile = get_file_name(&args).unwrap();

    let lines = read_lines(infile);
    let banks: Vec<Bank> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let answer = banks
        .iter()
        .map(|b| { b.max_jolts() })
        .reduce(|acc, j| { acc + j })
        .unwrap();

    let answer_two = banks
        .iter()
        .map(|b| { b.maxxer_jolts(12) })
        .reduce(|acc, j| { acc + j })
        .unwrap();

    println!("The total joltage is: {answer}");
    println!("The even bigger joltage is {answer_two}");
}

#[derive(Debug)]
enum ParseBankError {
    BadCharacter,
}

struct Bank(Vec<u32>);

impl Bank {
    pub fn max_jolts(&self) -> u32 {
        // the max jolts of a bank is essentially finding the partition
        // of the vector [first], [second] that maximizes 
        // 10 * max([first]) + max([second]) and then returning
        // the maximized value.
        (1..self.0.len()).map(|p| { self.max_jolts_helper(p)}).max().unwrap()
    }

    fn max_jolts_helper(&self, p: usize) -> u32 {
        let (first, second) = self.0.split_at(p);
        first.iter().max().unwrap_or(&0) * 10 
            + second.iter().max().unwrap_or(&0)
    }

    pub fn maxxer_jolts(&self, batteries_on: usize) -> u64 {
        let mut res: u64 = 0;
        // offset represents "where to start looking" for the next battery
        let mut offset = 0;
        for i in 0..batteries_on {
            // Find the most significant remaining digit-battery.
            // It's the largest number anywhere between the previous
            // battery and batteries_on - i from the end
            let mut max = self.0.get(offset).unwrap();
            for j in offset..(self.0.len()+1-batteries_on+i) {
                if self.0.get(j).unwrap() > max {
                    max = self.0.get(j).unwrap();
                    offset = j;
                }
            }
            offset += 1; // Next loop start looking one spot further.
            // This is the math to "insert" the number into the most significant
            // digit (base 10)
            res *= 10;
            res += *max as u64;
        }
        res
    }
}

impl FromStr for Bank {
    type Err = ParseBankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Option<Vec<u32>> = s.chars().map(|c| {
            c.to_digit(10)
        }).collect();

        match nums {
            Some(ns) => Ok(Bank(ns)),
            None => Err(ParseBankError::BadCharacter),
        }
    }
}