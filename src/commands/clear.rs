use crate::todo_file::TodoFileHandler;

pub fn clear_todos() {
    let mut file_handler = TodoFileHandler::new();

    if file_handler.clear_todos().is_ok() {
        println!("Todos cleared!");
    }
}
