use crate::todo_file::TodoFileHandler;

pub fn add_todo(todo: &Option<String>) {
    if let Some(todo) = todo {
        if todo.is_empty() {
            eprintln!("Cannot add empty todo");
            return ();
        }
        let mut file_handler = TodoFileHandler::new();

        if file_handler.write_to_todos(todo.to_string()).is_ok() {
            println!("Todo added!")
        }
    }
}
