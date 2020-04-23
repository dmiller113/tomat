extern crate cursive;
extern crate structopt;

mod add;
mod cli;
mod config;
mod list;
mod view;

// use std::time::{ Duration };
use crate::add::add_pomodoro;
use crate::cli::{Command, Opt};
use crate::config::get_configuration;
use crate::list::list_pomodoros;
use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    get_configuration().map(|config| {
        let Opt { cmd, task_name, .. } = args;

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
            }
            Command::List {
                show_ended_tasks: show_all,
            } => {
                list_pomodoros(show_all);
            }
        }
    }).map_err(|err| println!("{:?}", err));
}
