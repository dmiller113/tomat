use crate::config::Config;
use cursive::Cursive;

pub struct Console {
    console: Cursive,
}

impl Console {
    pub fn setup(config: Config) -> Console {
        let Config { theme, .. } = config;
        let mut console = Cursive::default();
        console.set_theme(theme);

        Console { console }
    }

    pub fn run(&mut self) -> &Console {
        self.console.run();
        self
    }
}
