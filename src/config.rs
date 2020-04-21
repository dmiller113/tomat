pub struct Config {
    pub duration: u32,
}

pub fn get_configuration() -> Config {
    Config { duration: 15 }
}
