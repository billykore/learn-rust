use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => read_todos(),
            "add" => add_todos(&args[2]),
            "help" | _ => help()
        }
    } else {
        help()
    }
}

fn read_todos() {
    let file = fs::read_to_string("todos.txt").expect("Failed to open file");
    for (i, line) in file.lines().enumerate() {
        println!("{}. {}", i + 1, line)
    }
}

#[test]
fn test_read_todos() {
    read_todos()
}

fn add_todos(todo: &str) {
    let file = File::options()
        .append(true)
        .open("todos.txt")
        .expect("Failed to open file");
    let mut writer = BufWriter::new(file);
    let content = format!("{}\n", todo);
    writer.write_all(content.as_bytes()).expect("Failed to read to file");
}

#[test]
fn test_add_todo() {
    add_todos("Eat lunch")
}

fn help() {
    println!("Simple todo CLI app.
Available commands:
    - list
        Display list of todos.
        Example: todo list
    - add [TODO]
        Add new todo to list.
        Example: todo add 'Do laundry'
    - help
        Display list of commands.
        Example: todo help
");
}

#[test]
fn test_help() {
    help()
}
