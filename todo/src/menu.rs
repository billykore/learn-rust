use std::io;
use std::io::BufRead;
use crate::todo::Todo;
use crate::todo_list::TodoList;

pub struct Menu {
    todo_list: TodoList,
    keep_going: bool,
}

impl Menu {
    pub fn new(todo_list: TodoList) -> Self {
        Menu {
            todo_list,
            keep_going: true,
        }
    }

    pub fn start(&mut self) {
        while self.keep_going {
            self.display();
            self.process_option()
        }
    }

    fn display(&self) {
        println!("Welcome to Todo List App
Select an option:
    1. Display Todo List
    2. Add Todo
    3. Delete Todo
    4. Complete Todo
    5. Reset
    0. Exit
        ")
    }

    fn process_option(&mut self) {
        let option = self.get_input("Select an option: ");
        self.use_option(option.as_str())
    }

    fn use_option(&mut self, option: &str) {
        match option {
            "1" => self.get_todo_list_option(),
            "2" => self.add_todo_option(),
            "3" => self.delete_todo_option(),
            "4" => self.complete_todo_option(),
            "5" => self.reset_option(),
            "0" => self.exit_option(),
            _ => self.display()
        }
    }

    fn get_todo_list_option(&self) {
        self.todo_list.get_all()
    }

    fn add_todo_option(&mut self) {
        let title = self.get_input("Title: ");
        let desc = self.get_input("Description: ");
        let todo = Todo::new(title.as_str(), desc.as_str());
        self.todo_list.add(todo)
    }

    fn delete_todo_option(&mut self) {
        let id = self.get_input("Todo id: ");
        self.todo_list.delete(id.parse::<usize>().unwrap())
    }

    fn complete_todo_option(&mut self) {
        let id = self.get_input("Todo id: ");
        self.todo_list.complete_todo(id.parse::<usize>().unwrap())
    }

    fn reset_option(&mut self) {
        self.todo_list.reset();
    }

    fn exit_option(&mut self) {
        self.keep_going = false
    }

    fn get_input(&self, title: &str) -> String {
        println!("{}", title);
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        println!();
        line
    }
}

#[test]
fn test_display_menu() {
    let todo_list = TodoList::new();
    let menu = Menu::new(todo_list);
    menu.display()
}
