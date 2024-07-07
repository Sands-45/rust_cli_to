//use std::io::Write;
use ::std::io::{stdin, stdout, Write};
use loading::Loading;
use std::fs::{read_to_string, OpenOptions};
use std::path::Path;

const LOCAL_DB: &str = "todos.db";

//Load todos from file
fn load_todos(file_path: &str) -> Vec<String> {
    let todos: Vec<String>;
    let loading = Loading::default();

    if Path::new(file_path).exists() {
        loading.text("Loading todos from file...");

        todos = read_to_string(file_path)
            .map(|content| {
                content
                    .lines()
                    .map(|line| line.trim().to_string()) // Trim each line before collecting
                    .filter(|line| !line.is_empty()) // Filter out empty lines
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        loading.success("Finished loading todos from file!");
    } else {
        todos = Vec::new();
    }

    loading.end();
    return todos;
}

//Save todos to file
fn save_todos(todos: &Vec<String>, file_path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Overwrite existing content
        .open(file_path)
        .expect("Error opening/creating todo file");

    for todo in todos {
        writeln!(file, "{}", todo).expect("Error writing to file");
    }
}

//Welcome user to the todo app
fn welcome() {
    println!("Hi user, welcome to the todo app\n");
    //println!("You have {} todos\n", todos.len());

    //Instructions for the user and commands
    list_commands();
}

// Add a todo to the list
fn add_to(todos: &mut Vec<String>) {
    println!("\nWhat would you like to do?\n");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    //Remove the newline character
    let input: String = input.to_string();

    //Add input to the todos
    todos.push(input);
    save_todos(&todos, LOCAL_DB);

    //Print updated list
    println!("Now you have {} todos\n", todos.len());
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo);
    }

    //Code Ex
    // todos.iter(): This creates an iterator over the elements of the todos vector.
    //iter().enumerate(): This transforms the iterator into a new iterator that yields pairs. Each pair consists of:
    // The index of the element (starting from 0).
    // A reference to the element itself. More like map in JS
}

//Remove a todo from the list
fn remove_todo(todos: &mut Vec<String>) {
    //Check if there are todos to remove
    if todos.len() == 0 {
        println!("\nYou don't have any todo to remove\n");
        return;
    }

    println!("\nWhich todo would you like to remove?\n");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    //Remove the newline character
    let input: usize = input.trim().parse().unwrap();

    //Remove the todo
    todos.remove(input - 1);
    save_todos(&todos, LOCAL_DB);

    //Print updated list
    println!("\nYou have {} todos left\n", todos.len());
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo);
    }
}

//List all todos
fn list_todos(todos: &Vec<String>) {
    println!("\nYou have {} todos\n", todos.len());
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {} \n", i + 1, todo);
    }
}

//List all commands
fn list_commands() {
    //Instructions for the user and commands
    println!("Here are the commands you can use:\n");
    //println!("0. Sync\n");
    println!("1. Add\n");
    println!("2. Remove\n");
    println!("3. List\n");
    println!("4. List commands\n");
    println!("5. Exit\n");
}

//Main Function
fn main() {
    //let mut todos: Vec<String> = Vec::new();
    let mut todos: Vec<String> = load_todos(LOCAL_DB);

    //Greet the user
    welcome();

    //Listen for user input
    loop {
        print!("Enter command: \n");
        // Make sure "Enter command" is printed before blocking on input
        stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "1" => add_to(&mut todos),
            "2" => remove_todo(&mut todos),
            "3" => list_todos(&todos),
            "4" => list_commands(),
            "5" => break,
            _ => println!("Invalid command, please try again."),
        }
    }
}
