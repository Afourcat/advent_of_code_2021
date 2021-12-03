use utils::get_input;

fn main() {
    let values = get_input::<String>();
    let bin_length = values[0].len();
    let gamma_str = compute_gamma_str(&values, bin_length);

    let (gamma, epsilon) = get_epsilon_gamma_as_number(gamma_str, bin_length);
    println!(
        "Gamma: {:#08b}\nEpsilon: {:#08b}\noutput: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn get_epsilon_gamma_as_number(gamma_str: String, bin_length: usize) -> (u32, u32) {
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let offset = 32 - bin_length;
    let mask = (u32::MAX << offset) >> offset;
    (gamma, mask ^ gamma)
}

fn compute_gamma_str(values: &[String], bin_length: usize) -> String {
    let mut gamma_str = String::with_capacity(bin_length);

    for i in 0..bin_length {
        let mut acc = 0;

        for j in 0..values.len() {
            if values[j].as_bytes()[i] == '1' as u8 {
                acc += 1;
            } else {
                acc -= 1;
            }
        }

        let char = if acc > 0 { '1' } else { '0' };
        gamma_str.push(char);
    }
    gamma_str
}
