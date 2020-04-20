extern crate structopt;

mod config;
mod cli;

// use std::time::{ Duration };
use structopt::StructOpt;
use crate::config::Config;
use crate::cli::{Opt, Command};

fn main() {
    let args = Opt::from_args();
    let config = Config {
        duration: 15,
    };
    let Opt {
        is_debug: _,
        cmd,
        task_name,
    } = args.clone();

    let subcommand = match (task_name, cmd) {
        (_, Some(c)) => c,
        (Some(name), None) => Command::Add {
            duration: config.duration,
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
