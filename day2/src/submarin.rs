use crate::command::Command;

#[derive(Default, Debug, Clone, Copy)]
pub struct SubMarin {
    depth: u32,
    horizontal_position: u32,
    aim: u32,
}

impl SubMarin {
    pub fn new() -> Self {
        SubMarin::default()
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Forward(x) => {
                self.horizontal_position += x;
                self.depth += x * self.aim;
            }
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
        }
    }

    pub fn print_state(&self) {
        println!(
            "The submarin final position:\n{:?}\nThe result is {} * {} = {}.",
            self,
            self.horizontal_position,
            self.depth,
            self.horizontal_position * self.depth
        );
    }
}
