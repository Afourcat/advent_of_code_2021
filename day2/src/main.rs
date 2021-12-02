use std::str::FromStr;

use utils::get_input;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let command_name = splitted.next().unwrap();
        let payload = splitted.next().unwrap().parse::<u32>().unwrap();

        let command = match command_name {
            "forward" => Command::Forward(payload),
            "up" => Command::Up(payload),
            "down" => Command::Down(payload),
            _ => return Err("Unknown command."),
        };

        Ok(command)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct SubMarin {
    depth: u32,
    horizontal_position: u32,
}

impl SubMarin {
    fn new() -> Self {
        SubMarin::default()
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.horizontal_position += x,
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x,
        }
    }
}

fn main() {
    let commands = get_input::<Command>();

    let mut sub = SubMarin::new();

    for cmd in commands {
        sub.execute(cmd);
    }

    println!(
        "The submarin final position:\n{:?}\nThe result is {} * {} = {}.",
        sub,
        sub.horizontal_position,
        sub.depth,
        sub.horizontal_position * sub.depth
    );
}
