use crate::todo_file::TodoFileHandler;

pub fn activate_todo(todo_selector: &Option<String>) {
    if let Some(todo) = todo_selector {
        if todo.is_empty() {
            return ();
        }

        let mut file_handler = TodoFileHandler::new();

        match todo.parse::<usize>() {
            Ok(todo_selector_num) => match file_handler.toggle_todo_active(todo_selector_num) {
                Ok(_) => println!("Set Todo is active!"),
                Err(err) => panic!("{err}"),
            },
            Err(err) => panic!("{err}"),
        };
    }
}
