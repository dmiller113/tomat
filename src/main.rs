use std::time::{ Duration };

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
        #[structopt(short = "m", long = "minutes", default_value = "15")]
        /// Duration of pomodoro, in minutes
        duration: u32,

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
        cmd,
        task_name,
    } = args.clone();

    let subcommand = match (task_name, cmd) {
        (_, Some(c)) => c,
        (Some(name), None) => Command::Add {
            duration: 15,
            task_name: name,
        },
        (None, None) => Command::List {
            show_ended_tasks: false,
        },
    };

    match subcommand {
        Command::Add {
            duration: _dur,
            task_name: _name,
        } => {
            println!("add");
        },
        Command::List {
            show_ended_tasks: _show_all,
        } => {
            println!("list");
        }
    }

    println!("We ran.");
}
