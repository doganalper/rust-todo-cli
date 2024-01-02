mod args;

use args::Args;

fn main() {
    let mut args = Args::new();

    match args.parse() {
        Ok(_) => {
            println!("command: {:?}", args.command);

            if let Some(args) = args.rest {
                for arg in args {
                    println!("Rest: {arg}");
                }
            }
        }
        Err(err) => eprintln!("{err}"),
    };
}
