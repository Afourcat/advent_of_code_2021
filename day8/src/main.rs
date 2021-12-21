#![allow(dead_code)]
use std::{arch::x86_64::_mm_andnot_ps, collections::HashMap, num, ops::Deref, str::FromStr};

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

    fn translate(&self, mapping: &HashMap<char, char>) -> String {
        self.inner
            .chars()
            .map(|letter| mapping.get(&letter).unwrap())
            .collect::<String>()
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
    // Compute the number of occurences of digits: 1, 4, 7, 8.
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

    fn solve_mapping(&self) -> HashMap<&str, u8> {
        let mut mapping: HashMap<char, char> = HashMap::with_capacity(7);
        let mut number_mapping: HashMap<&str, u8> = HashMap::with_capacity(9);

        let one = &self
            .digits
            .iter()
            .find(|e| e.inner.len() == 2)
            .unwrap()
            .inner;
        number_mapping.insert(&one, 1);

        let four = &self
            .digits
            .iter()
            .find(|e| e.inner.len() == 4)
            .unwrap()
            .inner;

        number_mapping.insert(&four, 4);
        let seven = &self
            .digits
            .iter()
            .find(|e| e.inner.len() == 3)
            .unwrap()
            .inner;

        number_mapping.insert(&seven, 7);
        let height = &self
            .digits
            .iter()
            .find(|e| e.inner.len() == 7)
            .unwrap()
            .inner;
        number_mapping.insert(&height, 8);

        Self::solve_a(&mut mapping, one.as_bytes(), seven.as_bytes());
        self.solve_3(&mapping, &mut number_mapping, one.as_bytes());
        self.solve_5(&mut number_mapping, four.as_bytes());
        self.solve_2(&mut number_mapping);
        self.solve_9(&mut number_mapping, four.as_bytes());
        self.solve_0(&mut number_mapping, one.as_bytes());
        self.solve_6(&mut number_mapping);

        number_mapping
    }

    fn solve_a(mapping: &mut HashMap<char, char>, one: &[u8], seven: &[u8]) {
        let rest = seven
            .iter()
            .cloned()
            .filter(|letter| !one.contains(letter))
            .next()
            .unwrap();

        mapping.insert('a', rest as char);
    }

    // Deduce the wiring and compute the output value.
    fn part2(&self) -> u32 {
        let mapping = self.solve_mapping();
        println!("Mapping {:?}", mapping);
        let mut buf: [u8; 4] = [0; 4];

        for (idx, digit) in self.outputs.iter().enumerate() {
            let mut sorted_digit = digit.inner.clone().into_bytes();
            sorted_digit.sort();

            let (key, value) = mapping
                .iter()
                .find(|(key, value)| {
                    let mut cloned_key = key.deref().deref().to_owned().into_bytes();
                    cloned_key.sort();
                    cloned_key == sorted_digit
                })
                .unwrap();
            buf[idx] = (*value) + 48;
        }
        let str_output = std::str::from_utf8(&buf).unwrap();
        println!("{}", str_output);
        str_output.parse().unwrap()
    }

    fn solve_6<'a>(&'a self, number_mapping: &mut HashMap<&'a str, u8>) {
        let digit = self
            .digits
            .iter()
            .find(|digit| !number_mapping.contains_key(digit.inner.as_str()));

        println!("Map 6 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 6);
    }

    fn solve_0<'a>(&'a self, number_mapping: &mut HashMap<&'a str, u8>, one: &[u8]) {
        let digit = self
            .digits
            .iter()
            .filter(|digit| digit.inner.len() == 6)
            .find(|digit| {
                let is_not_9 = number_mapping
                    .get(digit.inner.as_str())
                    .map_or(true, |value| *value != 9);

                let count = digit
                    .inner
                    .as_bytes()
                    .iter()
                    .filter(|digit| one.contains(digit))
                    .count();
                count == 2 && is_not_9
            });

        println!("Map 0 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 0);
    }

    fn solve_9<'a>(&'a self, number_mapping: &mut HashMap<&'a str, u8>, four: &[u8]) {
        let digit = self
            .digits
            .iter()
            .filter(|digit| digit.inner.len() == 6)
            .find(|digit| {
                digit
                    .inner
                    .as_bytes()
                    .iter()
                    .filter(|digit| four.contains(digit))
                    .count()
                    == 4
            });

        println!("Map 9 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 9);
    }

    fn solve_2<'a>(&'a self, number_mapping: &mut HashMap<&'a str, u8>) {
        let digit = self
            .digits
            .iter()
            .filter(|digit| digit.inner.len() == 5)
            .find(|digit| !number_mapping.contains_key(digit.inner.as_str()));

        println!("Map 2 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 2);
    }

    fn solve_5<'a>(&'a self, number_mapping: &mut HashMap<&'a str, u8>, four: &[u8]) {
        let digit = self
            .digits
            .iter()
            .filter(|digit| digit.inner.len() == 5)
            .find(|digit| {
                let count = digit
                    .inner
                    .as_bytes()
                    .iter()
                    .filter(|c| four.contains(c))
                    .count();
                !number_mapping.contains_key(digit.inner.as_str()) && count == 3
            });

        println!("Map 5 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 5);
    }

    fn solve_3<'a>(
        &'a self,
        mapping: &HashMap<char, char>,
        number_mapping: &mut HashMap<&'a str, u8>,
        one: &[u8],
    ) {
        let digit = self
            .digits
            .iter()
            .filter(|digit| digit.inner.len() == 5)
            .find(|digit| {
                let count = digit
                    .inner
                    .as_bytes()
                    .iter()
                    .filter(|c| one.contains(c))
                    .count();
                let a = mapping.get(&'a').unwrap();
                let contains_a_seg = digit.inner.as_bytes().contains(&(*a as u8));
                count == 2 && contains_a_seg
            });

        println!("Map 3 to {}", digit.unwrap().inner);
        number_mapping.insert(&digit.unwrap().inner, 3);
    }
}

fn main() {
    let lines = get_input::<Line>();
    let total = lines.into_iter().map(|line| line.part2()).sum::<u32>();
    println!("Occurences Total: {}", total);
}
