use crate::config::Config;
use cursive::Cursive;
use cursive::view::View;

pub mod start;

pub struct Console {
    console: Cursive,
}

impl Console {
    pub fn attach<V: View>(mut self, view: V) -> Self {
        self.console.add_layer(view);
        self
    }

    pub fn setup(config: Config) -> Self {
        let Config { theme, .. } = config;
        let mut console = Cursive::default();
        console.set_theme(theme);

        Console { console }
    }

    pub fn run(mut self) -> Self {
        self.console.run();
        self
    }
}

