use std::{cmp::Ordering, error::Error};

#[derive(Debug, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub status: bool,
    pub content: String,
}

fn parse_line(split_line: Vec<&str>) -> Result<(u32, bool, String), Box<dyn Error>> {
    let id = split_line[0].parse::<u32>()?;
    let status = !split_line[2].contains("❌");
    let content = split_line[3].to_owned();

    Ok((id, status, content))
}

impl Todo {
    pub fn new(id: u32, content: &String) -> Self {
        Todo {
            id,
            content: content.clone(),
            status: false,
        }
    }

    pub fn from(line: String) -> Self {
        let split: Vec<&str> = line.split_whitespace().collect();

        match parse_line(split) {
            Ok((id, status, content)) => Self {
                id,
                status,
                content,
            },
            Err(_) => panic!("Could not parse {line}"),
        }
    }

    pub fn as_line(&self, with_new_line: bool) -> String {
        let status_icon = if self.status { "✅" } else { "❌" };

        let mut line = format!("{} -> ({}) {}", self.id, status_icon, self.content);

        if with_new_line {
            line.push_str("\n");
        };

        line
    }

    pub fn decrease_index(&mut self) {
        if self.id.cmp(&0) == Ordering::Equal {
            return ();
        }

        self.id -= 1;
    }

    pub fn toggle_status(&mut self) {
        self.status = !self.status
    }
}

#[cfg(test)]
mod tests {
    use crate::todo::parse_line;

    use super::Todo;

    #[test]
    fn creates_new_todo_from_line() {
        let expected = Todo {
            id: 1,
            status: false,
            content: "Deneme".to_string(),
        };

        let compare_value = Todo::from("1 -> (❌) Deneme".to_string());

        assert_eq!(compare_value, expected);
    }

    #[test]
    fn parses_line() {
        let expected = (1, false, "Deneme".to_string());

        assert_eq!(
            parse_line(vec!["1", "->", "(❌)", "Deneme"]).unwrap(),
            expected
        );
    }

    #[test]
    fn decreases_id_higher_than_0() {
        let mut expected = Todo {
            id: 2,
            status: true,
            content: "Content".to_string(),
        };

        expected.decrease_index();

        assert_eq!(expected.id, 1);
    }

    #[test]
    fn doesnt_decreases_id_on_0() {
        let mut expected = Todo {
            id: 0,
            status: true,
            content: "Content".to_string(),
        };

        expected.decrease_index();

        assert_eq!(expected.id, 0);
    }

    #[test]
    fn makes_status_true() {
        let mut expected = Todo {
            id: 1,
            status: false,
            content: "Content".to_string(),
        };

        expected.toggle_status();

        assert_eq!(expected.status, true);
    }

    #[test]
    fn makes_status_false() {
        let mut expected = Todo {
            id: 1,
            status: true,
            content: "Content".to_string(),
        };

        expected.toggle_status();

        assert_eq!(expected.status, false);
    }
}
