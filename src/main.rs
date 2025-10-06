use std::{fs, u32};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

// Commandes: add tache, list, rm id, change_state id

fn initialize_json() {
    let data = fs::read_to_string("./src/todo.json").unwrap();
    if data.trim().is_empty() {
        fs::write("./src/todo.json", "[]").unwrap();
    }
}

fn add_todo(title: String){
    let data = fs::read_to_string("./src/todo.json").unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    if let Some(array) = json.as_array(){
        if array.is_empty() {
            let new_todo = Todo{id: 1, title: title, done: false};
            let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap();
            todos.push(new_todo);
            fs::write("./src/todo.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
            println!("Nouvelle tâche ajoutée !");
        }
        else {
            // recuperer id max + 1
            let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap();
            let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let new_todo = Todo{id: next_id, title: title, done: false};
            todos.push(new_todo);
            fs::write("./src/todo.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
            println!("Nouvelle tâche ajoutée !");
        }
    }
}

fn change_state(todo_id: u32){
    let data = fs::read_to_string("./src/todo.json").unwrap();
    let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == todo_id) {
        todo.done = !todo.done;
    }
    fs::write("./src/todo.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
}

fn get_todos(){
    let data = fs::read_to_string("./src/todo.json").unwrap();
    let todos: Vec<Todo> = serde_json::from_str(&data).unwrap();
    for todo in todos{
        println!("id: {} | tâche: {} | état:{}", todo.id, todo.title, todo.done)
    }
}

fn remove_todo(todo_id: u32){
    let data = fs::read_to_string("./src/todo.json").unwrap();
    let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap();
    if let Some(pos) = todos.iter().position(|t| t.id == todo_id) {
        todos.remove(pos);
    }
    fs::write("./src/todo.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
}

fn main(){
    initialize_json();
    loop {
        println!(" \n Commande (add <tache>, list, rm <id>, change_state <id>, exit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => break,
            "list" => get_todos(),
            _ if input.starts_with("add ") => {
                let title = input[4..].trim().to_string();
                add_todo(title);
            }
            _ if input.starts_with("rm ") => {
                if let Ok(id) = input[3..].trim().parse::<u32>(){
                    remove_todo(id);
                } else {
                    println!("Invalid ID");
                }
            }
            _ if input.starts_with("change_state ") => {
                if let Ok(id) = input[13..].trim().parse::<u32>() {
                    change_state(id);
                } else {
                    println!("Invalid ID");
                }
            }
            _ => println!("Commande non reconnue"),
        }
    }
}