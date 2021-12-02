use utils::get_input;

fn main() {
    let depth_mesurements = get_input::<u32>();
    println!(
        "Computing the steepness of the following input: {:?}.",
        depth_mesurements
    );

    mesure_steepness(&depth_mesurements);
    mesure_steepness_sliding_window::<3>(&depth_mesurements);
}

fn mesure_steepness(depth_mesurements: &[u32]) {
    let output =
        depth_mesurements.windows(2).fold(
            0,
            |acc, window| if window[0] < window[1] { acc + 1 } else { acc },
        );

    println!("Output: {}", output);
}

fn mesure_steepness_sliding_window<const N: usize>(depth_mesurements: &[u32]) {
    let output = depth_mesurements
        .windows(N)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(
            0,
            |acc, window| if window[0] < window[1] { acc + 1 } else { acc },
        );

    println!("Output sliding window: {}", output);
}
