use structopt::StructOpt;

#[derive(Clone, StructOpt)]
#[structopt(name = "tomat", about = "A simple, cli pomodoro timer")]
pub struct Opt {
    #[structopt(short = "d", long = "debug")]
    /// Show debug information
    pub is_debug: bool,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,

    #[structopt(name = "task name")]
    /// Name of pomodoro.
    pub task_name: Option<String>,
}

#[derive(Clone, StructOpt)]
pub enum Command {
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
    },
}
