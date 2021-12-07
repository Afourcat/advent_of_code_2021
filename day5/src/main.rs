use std::{fmt::format, slice::Iter, str::FromStr};

use utils::get_input;

#[derive(Debug, Default)]
struct Vents {
    start: (u32, u32),
    end: (u32, u32),
}

impl FromStr for Vents {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();

        let start_x = iter
            .by_ref()
            .take_while(|c| *c != ',')
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let start_y = iter
            .by_ref()
            .take_while(|c| *c != ' ')
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let end_x = iter
            .by_ref()
            .skip_while(|c| !c.is_digit(10))
            .take_while(|c| *c != ',')
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let end_y = iter.collect::<String>().parse::<u32>().unwrap();

        Ok(Vents {
            start: (start_x, start_y),
            end: (end_x, end_y),
        })
    }
}

fn main() {
    let vents = get_input::<Vents>();

    // Get the greatest value to infer the size of the map.
    let max = vents.iter().fold(0_usize, |acc, Vents { start, end }| {
        let max = start.0.max(start.1).max(end.0).max(end.1);
        if max as usize > acc {
            max as usize
        } else {
            acc
        }
    }) + 1;

    let mut table = vec![0; max * max];

    for vent in vents {
        increment_table(&mut table, max, &vent);
    }

    let nb_overlap = table.iter().filter(|e| **e > 1).count();
    println!("Nb overlap: {}", nb_overlap);
}

fn increment_table(table: &mut Vec<i32>, length: usize, vent: &Vents) {
    let ((x1, y1), (x2, y2)) = if vent.start.0 > vent.end.0 || vent.start.1 > vent.end.1 {
        (vent.end, vent.start)
    } else {
        (vent.start, vent.end)
    };

    if x1 == x2 || y1 == y2 {
        for x in x1..=x2 {
            for y in y1..=y2 {
                table[(y as usize * length) + x as usize] += 1;
            }
        }
    } else {
        make_diag(table, length, vent);
    }
}

fn make_diag(table: &mut Vec<i32>, length: usize, vent: &Vents) {
    let (x1, y1) = vent.start;
    let (x2, y2) = vent.end;

    let x_iter: Box<dyn Iterator<Item = u32>> = if x1 > x2 {
        Box::new((x2..=x1).rev().into_iter())
    } else {
        Box::new((x1..=x2).into_iter())
    };

    let y_iter: Box<dyn Iterator<Item = u32>> = if y1 > y2 {
        Box::new((y2..=y1).rev().into_iter())
    } else {
        Box::new((y1..=y2).into_iter())
    };

    for (x, y) in x_iter.zip(y_iter) {
        table[(y as usize * length) + x as usize] += 1;
    }
}
