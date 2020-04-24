use crate::config::Config;
use crate::view::Console;
use std::format;

pub fn start_pomodoro(duration: u32, task_name: String, config: Config) {
    let mut console = Console::setup(config)
       .add_alert(format!(
           "{} - started\nWorking for {} minutes",
           task_name,
           duration,
       ));

    console.run();
}
