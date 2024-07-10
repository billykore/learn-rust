use crate::menu::Menu;
use crate::todo_list::TodoList;

mod todo;
mod todo_list;
mod menu;

fn main() {
    let todo_list = TodoList::new();
    let mut menu = Menu::new(todo_list);
    menu.start()
}
