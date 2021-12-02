use utils::get_input;

mod command;
mod submarin;

fn main() {
    let commands = get_input::<command::Command>();
    let mut sub = submarin::SubMarin::new();

    for cmd in commands {
        sub.execute(cmd);
    }
    sub.print_state();
}
