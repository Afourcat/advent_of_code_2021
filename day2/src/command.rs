use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let command_name = splitted.next().ok_or("Invalid command")?;
        let payload = splitted
            .next()
            .ok_or("Invalid payload")?
            .parse::<u32>()
            .map_err(|_| "Invalid payload")?;

        let command = match command_name {
            "forward" => Command::Forward(payload),
            "up" => Command::Up(payload),
            "down" => Command::Down(payload),
            _ => return Err("Unknown command."),
        };

        Ok(command)
    }
}
