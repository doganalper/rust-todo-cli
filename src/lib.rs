mod file;
mod todo;

use core::panic;

use file::FileHandler;

pub enum COMMAND {
    ADD,
    LIST,
    CLEAR,
    DELETE,
}

pub fn run_command(command: &COMMAND, arg: Option<String>) {
    match command {
        COMMAND::ADD => add_todo(&arg),
        COMMAND::LIST => list_todos(),
        COMMAND::CLEAR => clear_todos(),
        COMMAND::DELETE => delete_todo(&arg),
    }
}

fn delete_todo(todo_selector: &Option<String>) {
    if let Some(todo) = todo_selector {
        if todo.is_empty() {
            return ();
        }
        println!("{:?}", todo_selector);
        let mut file_handler = FileHandler::new();

        match todo.parse::<usize>() {
            Ok(todo_selector_num) => match file_handler.delete_todo(todo_selector_num) {
                Ok(_) => println!("Todo deleted!"),
                Err(err) => panic!("{err}"),
            },
            Err(err) => panic!("{err}"),
        };
    };
}

fn clear_todos() {
    let mut file_handler = FileHandler::new();

    if file_handler.clear_todos().is_ok() {
        println!("Todos cleared!");
    }
}

fn list_todos() {
    let mut file_handler = FileHandler::new();

    if let Ok(todos_content) = file_handler.get_todos_content() {
        if todos_content.is_empty() {
            println!("You have no todos yet!");
        } else {
            println!("{todos_content}");
        }
    }
}

pub fn add_todo(todo: &Option<String>) {
    if let Some(todo) = todo {
        if todo.is_empty() {
            eprintln!("Cannot add empty todo");
            return ();
        }
        let mut file_handler = FileHandler::new();

        match file_handler.write_to_todos(todo.to_string()) {
            Ok(_) => println!("Wrote to file"),
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}