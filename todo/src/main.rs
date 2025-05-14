use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: usize,
    text: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: &str) {
        let task = Task {
            id: self.next_id,
            text: title.to_string(),
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.next_id += 1;
        self.tasks.push(task);
    }

    fn load_data(file_name: &str) -> Result<Self, String> {
        if !Path::new(file_name).exists() {
            println!("New file has been created!");
            File::create(file_name).map_err(|e| format!("Failed to create file: {}", e))?;
        }
        let mut file =
            File::open(file_name).map_err(|e| format!("Failed to Open the file: {}", e))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to create the file: {}", e))?;

        if content.trim().is_empty() {
            println!("File is empty!");
            return Ok(TodoList::new());
        }

        serde_json::from_str(&content).map_err(|e| format!("Failed to deserialize tasks: {}", e))
    }
}

fn main() {
    let file_name = "todos.json";

    let mut data: TodoList = match TodoList::load_data(file_name) {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error loading todo list: {}", err);
            TodoList::new()
        }
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "add" => {
                let text = args[2..].join(" ");
                data.add_task(&text);
                let json = serde_json::to_string_pretty(&data).unwrap();
                let mut file = File::create(file_name).unwrap();
                file.write_all(json.as_bytes()).unwrap();
                println!("Task added!");
            }
            "list" => {
                for (index, task) in data.tasks.iter().enumerate() {
                    println!(
                        "[{}] - {} (created: {})",
                        index,
                        task.text,
                        task.created_at.format("%Y-%m-%d %H:%M")
                    );
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    } else {
        println!("  add <task>       - Add a new task");
    }
}
