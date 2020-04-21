extern crate cursive;
extern crate structopt;

mod add;
mod config;
mod cli;
mod list;
mod view;

// use std::time::{ Duration };
use structopt::StructOpt;
use crate::add::{add_pomodoro};
use crate::config::{get_configuration};
use crate::cli::{Opt, Command};
use crate::list::{list_pomodoros};

fn main() {
    let args = Opt::from_args();
    let config = get_configuration();
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
            duration: dur,
            task_name: name,
        } => {
            add_pomodoro(dur, name);
        },
        Command::List {
            show_ended_tasks: show_all,
        } => {
            list_pomodoros(show_all);
        }
    }
}
