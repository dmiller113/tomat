use crate::types::Pomodoro;
use cursive::Printer;
use cursive::vec::Vec2;
use cursive::view::View;
use std::time::Instant;

pub enum Error {
    MissingPomodoros,
}

pub struct StartView {
    // List of upcoming pomodoros
    upcoming: Vec<Pomodoro>,

    // Current pomodoro
    current: Pomodoro,

    // Time this view was started at
    created_at: Instant,
}

impl StartView {
    pub fn new(pomodoros: Vec<Pomodoro>) -> Result<Self, Error> {
        match pomodoros.split_first() {
            Some((first, rest)) =>
                Ok(StartView {
                    created_at: Instant::now(),
                    current: first.to_owned(),
                    upcoming: rest.to_vec(),
                }),
            None => Err(Error::MissingPomodoros), 
        }
    }
}

impl View for StartView {
    fn draw(&self, printer: &Printer) {
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(2,2)
    }
}
