use crate::todo_file::TodoFileHandler;

pub fn list_todos() {
    let mut file_handler = TodoFileHandler::new();

    if let Ok(todos_content) = file_handler.get_todos_content() {
        if todos_content.is_empty() {
            println!("You have no todos yet!");
        } else {
            println!("{todos_content}");
        }
    }
}
