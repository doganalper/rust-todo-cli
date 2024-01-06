mod args;
mod commands;
mod todo;
mod todo_file;

use args::Args;
use commands::run_command;

fn main() {
    let mut args = Args::new();

    match args.parse() {
        Ok(_) => {
            run_command(&(args.command.unwrap()), args.rest);
        }
        Err(err) => eprintln!("{err}"),
    };
}
