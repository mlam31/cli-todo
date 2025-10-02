use std::{fs};
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

fn initialize_json() {
    let data = fs::read_to_string("./src/todo.json").unwrap();
    if data.trim().is_empty() {
        fs::write("./src/todo.json", "[]").unwrap();
    }
}

fn create_todo(title: String){
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

fn main(){
    initialize_json();
    create_todo(String::from("Faire à manger"));
    create_todo(String::from("Faire les courses"));
    create_todo(String::from("Faire du sport"));
}