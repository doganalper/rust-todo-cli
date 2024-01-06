mod activate;
mod add;
mod clear;
mod delete;
mod list;
mod toggle;
mod active;

use activate::activate_todo;
use add::add_todo;
use clear::clear_todos;
use delete::delete_todo;
use list::list_todos;
use toggle::toggle_todo;
use active::display_active;

pub enum COMMAND {
    ADD,
    LIST,
    CLEAR,
    DELETE,
    TOGGLE,
    ACTIVATE,
    ACTIVE,
}

pub fn run_command(command: &COMMAND, arg: Option<String>) {
    match command {
        COMMAND::ADD => add_todo(&arg),
        COMMAND::LIST => list_todos(),
        COMMAND::CLEAR => clear_todos(),
        COMMAND::DELETE => delete_todo(&arg),
        COMMAND::TOGGLE => toggle_todo(&arg),
        COMMAND::ACTIVATE => activate_todo(&arg),
        COMMAND::ACTIVE => display_active(),
    }
}
