use crate::view::Console;

pub fn add_pomodoro(duration: u32, name: String) {
    let mut console = Console::setup();
    console.run();

    println!("add, duration: {} name: {}", duration, name);
}
