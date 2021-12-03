use utils::get_input;

fn main() {
    let strings: Vec<String> = get_input();

    let oxygen = get(strings.clone(), 0, |acc| acc >= 0);
    println!("Oxygen: {:?}", oxygen);
    let co2 = get(strings, 0, |acc| acc < 0);
    println!("CO2: {:?}", co2);
    println!("Output: {}", co2 * oxygen);
}

fn get<P: Fn(i32) -> bool>(strings: Vec<String>, idx: usize, predicate: P) -> u32 {
    let acc = get_most_common(&strings, idx);
    let char = if predicate(acc) { '1' } else { '0' };
    let output = strings
        .into_iter()
        .filter(|string| string.as_bytes()[idx] == char as u8)
        .collect::<Vec<String>>();

    if output.len() == 1 {
        u32::from_str_radix(&output[0], 2).unwrap()
    } else {
        get(output, idx + 1, predicate)
    }
}

fn get_most_common(strings: &[String], idx: usize) -> i32 {
    let mut acc = 0;

    for string in strings {
        let char = string.as_bytes()[idx];
        acc += if char == '1' as u8 { 1 } else { -1 }
    }
    acc
}
