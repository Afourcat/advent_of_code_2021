#![allow(dead_code)]
use std::str::FromStr;

use utils::get_input;

#[derive(Debug)]
struct Digit {
    inner: String,
}

impl Digit {
    fn new<T>(from: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            inner: from.as_ref().to_string(),
        }
    }
}

#[derive(Debug)]
struct Line {
    digits: Vec<Digit>,
    outputs: Vec<Digit>,
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('|').collect();
        let digits_str = splitted[0];
        let digits = digits_str.split_whitespace().map(Digit::new).collect();

        let output_str = splitted[1];
        let outputs = output_str.split_whitespace().map(Digit::new).collect();

        Ok(Self { digits, outputs })
    }
}

impl Line {
    fn part1(&self) -> u32 {
        let mut occurences: [u32; 10] = [0; 10];

        for digit in &self.outputs {
            match digit.inner.len() {
                2 => occurences[1] += 1,
                4 => occurences[4] += 1,
                3 => occurences[7] += 1,
                7 => occurences[8] += 1,
                _ => (),
            }
        }
        println!("Occurences: {:?}", occurences);
        let total = occurences[1] + occurences[4] + occurences[7] + occurences[8];
        total
    }
}

fn main() {
    let lines = get_input::<Line>();
    let total = lines.into_iter().map(|line| line.part1()).sum::<u32>();
    println!("Occurences Total: {}", total);
}
