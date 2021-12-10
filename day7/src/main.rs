#![feature(int_abs_diff)]
use utils::get_input;

fn main() {
    let inputs = get_input::<String>();
    let split: Vec<&str> = inputs
        .get(0)
        .expect("We need at least one line.")
        .split(",")
        .collect();

    let positions: Vec<u32> = split
        .iter()
        .filter_map(|e| (*e).parse::<u32>().ok())
        .collect();

    // We may just find the minimum and the maximum of the
    let (min, max) = (
        *positions.iter().min().expect("No min value."),
        *positions.iter().max().expect("No max value."),
    );

    // Taking a middle slice of positions in order to save time.
    let fuel: u32 = (min..max)
        .map(|tested_position| {
            positions
                .iter()
                .map(|p| (1..=p.abs_diff(tested_position)).sum::<u32>())
                .sum()
        })
        .min()
        .expect("Error no minimum fuel usage.");
    println!("Min fuel: {}", fuel);
}
