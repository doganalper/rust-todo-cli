use crate::todo_file::TodoFileHandler;

pub fn display_active() {
    let mut file_handler = TodoFileHandler::new();

    match file_handler.get_active_todo() {
        Some(active_todo) => println!("{active_todo}"),
        None => println!("No active todo!"),
    }
}
