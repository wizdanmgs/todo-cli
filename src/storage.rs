use crate::todo::Todo;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "todo.json";

pub fn load_todos() -> Vec<Todo> {
    if !Path::new(FILE_PATH).exists() {
        return vec![];
    };
    let data = fs::read_to_string(FILE_PATH).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

pub fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize");
    fs::write(FILE_PATH, data).expect("Failed to write file");
}
