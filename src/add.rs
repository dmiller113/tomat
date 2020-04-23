use crate::config::Config;
use crate::view::Console;

pub fn add_pomodoro(duration: u32, name: String, config: Config) {
    let mut console = Console::setup(config);
    console.run();

    println!("add, duration: {} name: {}", duration, name);
}
