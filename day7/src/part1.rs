#![feature(int_abs_diff)]
use utils::get_input;

fn main() {
    let inputs = get_input::<String>();
    let split: Vec<&str> = inputs
        .get(0)
        .expect("We need at least one line.")
        .split(",")
        .collect();

    let length = split.len();

    let mut positions: Vec<u32> = split
        .iter()
        .filter_map(|e| (*e).parse::<u32>().ok())
        .collect();
    positions.sort();
    let mediane = positions[length / 2];
    let fuel: u32 = positions.iter().map(|p| p.abs_diff(mediane)).sum();
    println!("Fuel: {}", fuel);
}
