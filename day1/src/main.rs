use utils::get_input;

fn main() {
    let depth_mesurements: Vec<u32> = get_input::<u32>();

    println!(
        "Computing the steep of the following input: {:?}.",
        depth_mesurements
    );

    let output =
        depth_mesurements.windows(2).fold(
            0,
            |acc, window| if window[0] < window[1] { acc + 1 } else { acc },
        );

    println!("Output: {}", output);
}
