use crate::todo_file::TodoFileHandler;

pub fn delete_todo(todo_selector: &Option<String>) {
    if let Some(todo) = todo_selector {
        if todo.is_empty() {
            return ();
        }
        let mut file_handler = TodoFileHandler::new();

        match todo.parse::<usize>() {
            Ok(todo_selector_num) => match file_handler.delete_todo(todo_selector_num) {
                Ok(_) => println!("Todo deleted!"),
                Err(err) => panic!("{err}"),
            },
            Err(err) => panic!("{err}"),
        };
    };
}
