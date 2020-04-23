use crate::config::Config;
use crate::view::Console;

pub fn start_pomodoro(duration: u32, task_name: String, config: Config) {
    let mut console = Console::setup(config);
    console.run();
}
