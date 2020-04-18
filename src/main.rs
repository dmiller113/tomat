use std::time::{ Duration };
use std::thread::sleep;

extern crate structopt;

use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "tomat", about = "A simple, cli pomodoro timer")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    /// Show debug information
    is_debug: bool,

    /// Name of pomodoro.
    pomodoro_name: String,
}

fn main() {
    let args = Opt::from_args();
    let Opt { pomodoro_name, is_debug } = args.clone();

    if is_debug {
       println!("Args: {:#?}", args); 
    }

    let duration_in_minutes = 25;

    println!(
        "Starting pomodoro {}! See you in {} minutes.",
        pomodoro_name,
        duration_in_minutes
    );

    sleep(Duration::from_secs(duration_in_minutes * 60));
    println!("ring ring ring! {} is done!", pomodoro_name);
}
