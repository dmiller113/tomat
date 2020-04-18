use std::time::{ Duration };
use std::thread::sleep;

extern crate structopt;

use structopt::StructOpt;

#[derive(Clone, StructOpt)]
#[structopt(name = "tomat", about = "A simple, cli pomodoro timer")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    /// Show debug information
    is_debug: bool,

    #[structopt(subcommand)]
    cmd: Option<Command>,

    #[structopt(name = "task name")]
    /// Name of pomodoro.
    task_name: Option<String>,
}

#[derive(Clone, StructOpt)]
enum Command {
    /// Add to the list of active pomodoro
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "task name")]
        /// Name of pomodoro.
        task_name: String,
    },

    /// Show list of active pomodoro
    #[structopt(name = "list")]
    List {
        #[structopt(short = "a", long = "all")]
        /// Show cancelled and finished pomodoros
        show_ended_tasks: bool,
    }
}

fn main() {
    let args = Opt::from_args();
    let Opt {
        is_debug: _,
        cmd: _,
        task_name,
    } = args.clone();

    let duration_in_minutes = 25;

    task_name.map(|name| {
        println!(
            "Starting pomodoro {}! See you in {} minutes.",
            name,
            duration_in_minutes
        );

        sleep(Duration::from_secs(duration_in_minutes * 60));
        println!("ring ring ring! {} is done!", name);
        name
    });
}
