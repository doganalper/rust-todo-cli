use std::{
    cmp::Ordering,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    usize,
};

use crate::todo::Todo;

pub struct TodoFileHandler {
    path: Option<PathBuf>,
}

impl TodoFileHandler {
    pub fn new() -> Self {
        Self { path: None }
    }

    fn get_todo_file_path(&mut self) -> Result<(), &'static str> {
        if self.path.is_some() {
            return Ok(());
        }

        match home::home_dir() {
            Some(mut path) if !path.as_os_str().is_empty() => {
                path.push("todos");
                self.path = Some(path);

                Ok(())
            }
            _ => Err("Unable to get your home dir!"),
        }
    }

    fn create_todos_file(&self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = &self.path {
            File::create(path)?;
        }

        Ok(())
    }

    fn file_exists(&mut self) -> bool {
        match self.get_todo_file_path() {
            Ok(_) => match &self.path {
                Some(path) => Path::exists(path.as_path()),
                None => false,
            },
            Err(_) => false,
        }
    }

    fn get_todo_count(&mut self) -> u32 {
        if let Ok(content) = self.get_todos_content() {
            let mut line_count: u32 = 1;
            content.lines().for_each(|line| {
                if !line.is_empty() {
                    line_count += 1
                }
            });

            line_count
        } else {
            1
        }
    }

    pub fn get_todos_content(&mut self) -> Result<String, Box<dyn Error>> {
        if !self.file_exists() {
            self.create_todos_file().unwrap();
        }

        let content = fs::read_to_string(self.path.clone().unwrap())?;

        Ok(content)
    }

    pub fn clear_todos(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.file_exists() {
            panic!("No todos exist.");
        }

        fs::write(self.path.take().unwrap(), "")?;

        Ok(())
    }

    pub fn write_to_todos(&mut self, todo: String) -> Result<(), Box<dyn Error>> {
        if !self.file_exists() {
            self.create_todos_file()?;
        }
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .open(self.path.take().unwrap())?;

        let todo_count = self.get_todo_count().to_string();

        let todo_with_count = format!("{} -> (❌) {}\n", todo_count, todo);
        file.write_all(todo_with_count.as_bytes())?;

        Ok(())
    }

    pub fn delete_todo(&mut self, line_num: usize) -> Result<(), Box<dyn Error>> {
        if !self.file_exists() {
            panic!("No todos exist.");
        }

        let mut new_content = String::new();
        for (idx, line) in self.get_todos_content()?.lines().enumerate() {
            let mut todo = Todo::from(line.to_string());
            match idx.cmp(&(line_num - 1)) {
                Ordering::Less => {}
                Ordering::Equal => continue,
                Ordering::Greater => {
                    todo.decrease_index();
                }
            }

            new_content.push_str(&todo.as_line(true));
        }

        fs::write(self.path.take().unwrap(), new_content)?;

        Ok(())
    }
}
