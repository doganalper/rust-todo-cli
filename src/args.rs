use core::panic;
use std::env;

use todo::COMMAND;

pub struct Args {
    pub command: Option<COMMAND>,
    pub rest: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Args {
            command: None,
            rest: None,
        }
    }

    pub fn parse(&mut self) -> Result<(), &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            return Err("At least one command needs to be passed!");
        }

        let (command, rest) = self.seperate_args(args);

        self.command = match command.as_str() {
            "add" => Some(COMMAND::ADD),
            "list" => Some(COMMAND::LIST),
            "clear" => Some(COMMAND::CLEAR),
            "remove" => Some(COMMAND::DELETE),
            "toggle" => Some(COMMAND::TOGGLE),
            _ => panic!("This command is not supported"),
        };
        self.rest = Some(rest);

        Ok(())
    }

    fn seperate_args(&self, args: Vec<String>) -> (String, String) {
        let command = args[1].clone();

        let mut rest = Vec::new();
        for (_, arg) in args.clone().into_iter().enumerate().filter(|(i, _)| i > &1) {
            rest.push(arg)
        }

        (command, rest.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn should_seperate_multiple_args() {
        let args = Args::new();

        let expected_result = (String::from("add"), String::from("test1 test2"));
        let actual_result = args.seperate_args(vec![
            String::from("test/file/path"),
            String::from("add"),
            String::from("test1"),
            String::from("test2"),
        ]);

        assert_eq!(expected_result, actual_result);
    }
}
