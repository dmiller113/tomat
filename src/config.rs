pub struct Config {
    pub duration: u32,
}

pub fn get_configuration() -> Config {
    let config = Config {
        duration: 15,
    };
    config
}
