use std::fmt::{Debug, Display};

#[derive(Debug, Default, Clone, Copy)]
pub struct Square {
    pub value: u8,
    pub marked: bool,
}

impl Square {
    fn new(value: u8) -> Self {
        Square {
            value,
            ..Default::default()
        }
    }
}
#[derive(Debug)]
pub struct Board<const N: usize>
where
    [(); N * N]: Sized,
{
    pub inner: [Square; N * N],
    pub won: bool,
}

impl<const N: usize> Board<N>
where
    [(); N * N]: Sized,
{
    pub fn new() -> Self {
        Board {
            inner: [Square::default(); N * N],
            won: false,
        }
    }

    pub fn fill<T>(&mut self, lines: &[T])
    where
        T: AsRef<str>,
    {
        for (line_idx, line) in lines.iter().enumerate() {
            let numbers = line.as_ref().split_whitespace();
            for (idx, number) in numbers.enumerate() {
                let nbr = number.parse::<u8>().expect("Invalid number.");
                self.inner[(line_idx * N) + idx] = Square::new(nbr);
            }
        }
    }

    pub fn mark(&mut self, number: u8) {
        if let Some(ref mut square) = self.inner.iter_mut().find(|e| e.value == number) {
            square.marked = true;
        }
    }

    pub fn has_won(&self) -> bool {
        if self.won || (self.won_horizontaly() || self.won_verticaly()) {
            true
        } else {
            false
        }
    }

    fn won_horizontaly(&self) -> bool {
        self.inner
            .chunks(N)
            .any(|line| line.iter().all(|Square { marked, .. }| *marked))
    }

    fn won_verticaly(&self) -> bool {
        (0..N - 1).into_iter().any(|index| {
            self.inner
                .iter()
                .skip(index)
                .step_by(N)
                .all(|Square { marked, .. }| *marked)
        })
    }

    pub fn get_score(&self) -> usize {
        self.inner
            .iter()
            .filter_map(
                |Square { value, marked }| if *marked { None } else { Some(*value as usize) },
            )
            .sum()
    }
}

impl<const N: usize> Display for Board<N>
where
    [(); N * N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_str = self
            .inner
            .chunks(N)
            .map(|window| {
                window
                    .iter()
                    .map(|Square { value, marked }| {
                        if *marked {
                            format!("[{:>2}]", value)
                        } else {
                            format!("{:>2}", value)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", format_str)
    }
}

impl<T, const N: usize> From<&[T]> for Board<N>
where
    T: AsRef<str> + Sized,
    [(); N * N]: Sized,
{
    fn from(lines: &[T]) -> Board<N> {
        let mut board = Board::new();
        board.fill(lines);
        board
    }
}
