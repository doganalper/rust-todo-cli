mod args;

use args::Args;
use todo::run_command;

fn main() {
    let mut args = Args::new();

    match args.parse() {
        Ok(_) => {
            run_command(&(args.command.unwrap()), args.rest);
        }
        Err(err) => eprintln!("{err}"),
    };
}
