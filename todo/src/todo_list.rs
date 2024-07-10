use crate::todo::Todo;

pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.todos.len()
    }

    pub fn get_all(&self) {
        println!("List of Todos");
        if self.len() == 0 {
            println!("(empty)\n")
        }
        for todo in &self.todos {
            todo.print()
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }

    pub fn delete(&mut self, id: usize) {
        _ = self.todos.remove(id)
    }

    pub fn complete_todo(&mut self, id: usize) {
        self.todos[id].set_completed(true)
    }
}


#[test]
fn test_new_todo_list() {
    let list = TodoList::new();
    assert_eq!(0, list.todos.len())
}

#[test]
fn get_all() {
    let mut list = TodoList::new();
    list.get_all();

    list.add(Todo::new("Title", "Description"));
    list.add(Todo::new("Title", "Description"));
    list.get_all()
}

#[test]
fn test_add() {
    let mut list = TodoList::new();
    list.add(Todo::new("Title", "Description"));
    list.add(Todo::new("Title", "Description"));
    list.add(Todo::new("Title", "Description"));
    assert_eq!(3, list.len())
}

#[test]
fn test_delete() {
    let mut list = TodoList::new();
    list.add(Todo::new("Title", "Description"));
    assert_eq!(1, list.len());

    list.delete(0);
    assert_eq!(0, list.len())
}

