use crate::config::Config;
use crate::view::Console;
use crate::views::start::StartView;
use crate::types::Pomodoro;
use cursive::views::TextView;

pub fn start_pomodoro(pomodoro: Pomodoro, config: Config) {
    let console = match StartView::new(vec!(pomodoro)) {
        Ok(view) => {
            let con = Console::setup(config);
            con.attach(view)
        },
        Err(_) => {
            let con = Console::setup(config);
            con.attach(TextView::new("Error making start view"))
        }
    };
    console.run();
}
