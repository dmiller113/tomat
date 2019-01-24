#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tomat", about = "A simple, cli pomodoro timer")]
struct Opt {
    // is this a break
    #[structopt(short = "b", long = "break")]
    /// This pomodoro is a long break
    is_break: bool,

    pomodoro_name: String,
}

fn main() {
    let args = Opt::from_args();
    println!("Args; {:?}", args);
}
