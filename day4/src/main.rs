#![warn(dead_code)] // TODO(Afourcat) Remove
#![feature(drain_filter)]
#![feature(generic_const_exprs)]

mod board;
use board::Board;

use utils::get_input;

fn main() {
    let inputs: Vec<String> = get_input::<String>()
        .drain_filter(|line| !line.is_empty())
        .collect();

    let boards = get_boards::<5>(&inputs[1..]);
    let rollings = inputs[0]
        .split(",")
        .filter_map(|val| val.parse::<u8>().ok());

    play(rollings, boards);
}

fn play<const N: usize>(rollings: impl Iterator<Item = u8>, mut boards: Vec<Board<N>>)
where
    [(); N * N]: Sized,
{
    for roll in rollings {
        for board in &mut boards {
            board.mark(roll);
        }
        boards.drain_filter(|winner| {
            if winner.has_won() {
                println!(
                    "Winner is:\n{}\nWith value: {} * {} = {}\n",
                    winner,
                    winner.get_score(),
                    roll,
                    winner.get_score() * roll as usize
                );
                true
            } else {
                false
            }
        });
        if boards.is_empty() {
            break;
        }
    }
}

fn get_boards<const N: usize>(inputs: &[String]) -> Vec<Board<N>>
where
    [(); N * N]: Sized,
{
    inputs.chunks(N).map(|lines| Board::from(lines)).collect()
}
