# Tomat

This is very much in progress. I would not recommend using this until many versions down the line.

### Example
```
$> tomat "Write the codes"
```

### Usage
```
tomat [FLAGS] [task name] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -d, --debug      Show debug information
    -V, --version    Prints version information

ARGS:
    <task name>    Name of pomodoro.

SUBCOMMANDS:
    add     Add to the list of active pomodoro
    help    Prints this message or the help of the given subcommand(s)
    list    Show list of active pomodoro
		start   Start a pomodoro
```

When providing a task name, `start` is the default command. Without a task name, `list` is the default command.
