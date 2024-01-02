use std::env;

pub struct Args {
    pub command: Option<String>,
    pub rest: Option<Vec<String>>,
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

        self.command = Some(command);
        self.rest = Some(rest);

        Ok(())
    }

    fn seperate_args(&self, args: Vec<String>) -> (String, Vec<String>) {
        let command = args[1].clone();
        let rest = args
            .clone()
            .iter()
            .enumerate()
            .filter(|(i, _)| i > &1)
            .map(|(_, e)| e.clone())
            .collect();

        (command, rest)
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn should_seperate_multiple_args() {
        let args = Args::new();

        let expected_result = (
            String::from("add"),
            vec![String::from("test1"), String::from("test2")],
        );
        let actual_result = args.seperate_args(vec![
            String::from("test"),
            String::from("add"),
            String::from("test1"),
            String::from("test2"),
        ]);

        assert_eq!(expected_result, actual_result);
    }
}
