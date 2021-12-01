use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn get_input<T>() -> Vec<T>
where
    T: FromStr,
{
    let mut args_iter = std::env::args().skip(2);
    let file_path = args_iter.next().expect("An argument must be provided.");

    println!("Trying to open file {}.", file_path);
    if let Ok(file) = std::fs::File::open(file_path) {
        let buf = BufReader::new(file);
        buf.lines().filter_map(parse_line).collect()
    } else {
        println!("File not found. Using raw arguments.");
        std::env::args()
            .skip(2)
            .filter_map(|f| f.parse().ok())
            .collect()
    }
}

fn parse_line<T: FromStr>(line: Result<String, std::io::Error>) -> Option<T> {
    line.ok().and_then(|l| l.parse::<T>().ok())
}
