use crate::todo::Todo;

pub fn add_todo(todos: &mut Vec<Todo>, title: String) {
    let id = todos.len() as u32 + 1;
    todos.push(Todo {
        id,
        title,
        completed: false,
    });
}

pub fn list_todos(todos: &Vec<Todo>) {
    for todo in todos {
        let status = if todo.completed { "✔" } else { " " };
        println!("[{}] {} - {}", status, todo.id, todo.title);
    }
}

pub fn mark_done(todos: &mut [Todo], id: u32) -> bool {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        return true;
    }
    false
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: u32) -> bool {
    let original_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == original_len {
        return false;
    }

    reassign_ids(todos);
    true
}

fn reassign_ids(todos: &mut [Todo]) {
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.id = (index + 1) as u32;
    }
}
