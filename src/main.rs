#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tomat", about = "A simple, cli pomodoro timer")]
enum Timers {
    /// Timer Type
    #[structopt(name = "timer")]
    Timer {
        #[structopt(parse(from_str))]
        timer_type: String
    }
}

fn main() {
    let args = Timers::from_args();
    println!("Args; {:?}", args);
}
