pub struct Todo {
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: &str, description: &str) -> Self {
        Todo {
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn print(&self) {
        println!("Title: {} \nDescription: {} \nCompleted: {}\n", self.title, self.description, self.completed)
    }
}

#[test]
fn test_new_todo() {
    let todo = Todo::new("Eat lunch", "Don't be starving");
    assert_eq!("Eat lunch".to_string(), todo.title);
    assert_eq!("Don't be starving".to_string(), todo.description);
    assert_eq!(false, todo.completed);
}

#[test]
fn test_set_completed_todo() {
    let mut todo = Todo::new("Eat lunch", "Don't be starving");
    todo.set_completed(true);
    assert_eq!(true, todo.completed)
}

#[test]
fn test_print_todo() {
    let todo = Todo::new("Eat lunch", "Don't be starving");
    todo.print()
}
