use crate::config::Config;
use cursive::Cursive;
use cursive::views::Dialog;

pub struct Console {
    console: Cursive,
}

impl Console {
    pub fn add_alert(mut self, inner_text: String) -> Console {
        self.console.add_layer(Dialog::text(inner_text)
            .title("tomat")
            .button("Ok", |s| s.quit()));
        self
    }

    pub fn setup(config: Config) -> Console {
        let Config { theme, .. } = config;
        let mut console = Cursive::default();
        console.set_theme(theme);

        Console { console }
    }

    pub fn run(mut self) -> Console {
        self.console.run();
        self
    }
}

