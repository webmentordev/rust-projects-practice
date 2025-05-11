use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoList {
    tasks: Vec<Task>,
}

fn main() {
    let file_name = "todos.json";
    if !Path::new(file_name).exists() {
        println!("New file has been created!");
        File::create("todos.json").unwrap();
    }

    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.trim().is_empty() {
        println!("File is empty!");
    }

    let args: Vec<String> = env::args().collect();
    let mut data: TodoList = if content.trim().is_empty() {
        TodoList { tasks: vec![] }
    } else {
        serde_json::from_str(&content).unwrap()
    };

    if args.len() > 1 {
        let text = args[1..].join(" ");
        let task = Task { text };
        data.tasks.push(task);

        let json = serde_json::to_string_pretty(&data).unwrap();
        let mut file = File::create(file_name).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        println!("Task added!");
    }

    for (index, task) in data.tasks.iter().enumerate() {
        println!("[{}] - {}", index, task.text);
    }
}
