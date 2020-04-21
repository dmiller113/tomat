use cursive::Cursive;

pub struct Console {
    console: Cursive,
}

impl Console {
    pub fn setup() -> Console {
        Console {
            console: Cursive::default(),
        }
    }

    pub fn run(&mut self) -> &Console {
        self.console.run();
        self
    }
}
