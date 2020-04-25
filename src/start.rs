use crate::config::Config;
use crate::view::Console;
use crate::types::Pomodoro;
use std::format;
use std::time::Instant;


struct StartView {
    // List of upcoming pomodoros
    upcoming: Vec<Pomodoro>,

    // Current pomodoro
    current: Pomodoro,

    // Time this view was started at
    created_at: Instant,
}

impl StartView {
    pub fn new(pomodoros: Vec<Pomodoro>) -> Self {
        let (current, upcoming) = match pomodoros.split_first() {
            Some((first, rest)) => (first.to_owned(), rest.to_vec()),
            None => panic!("Oops")
        };
        StartView {
            created_at: Instant::now(),
            current,
            upcoming,
        }
    }
}

pub fn start_pomodoro(duration: u32, task_name: String, config: Config) {
    let mut console = Console::setup(config)
       .add_alert(format!(
           "{} - started\nWorking for {} minutes",
           task_name,
           duration,
       ));

    console.run();
}
