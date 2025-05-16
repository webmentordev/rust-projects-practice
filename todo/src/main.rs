use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

#[derive(Deserialize, Serialize)]
struct Task {
    id: usize,
    text: String,
    is_completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

#[derive(Deserialize, Serialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

fn main() {
    let file_name = "todos.json";

    // File Check & Create if not exist!
    if !Path::new(file_name).exists() {
        File::create(file_name).unwrap();
        println!("File {file_name} did not exist! so it has been created!");
    }

    // Read File and Data
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.trim().is_empty() {
        println!("File is empty!");
    }

    // Parse Data to string
    let mut todos_data = match serde_json::from_str(&content) {
        Ok(list) => list,
        Err(_) => TodoList {
            tasks: Vec::new(),
            next_id: 1,
        },
    };

    // Arguments for Data Operations!
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("  list               -  to list all tasks!");
        println!("  add <task>         -  to add a new task!");
        println!("  complete <task>    -  mark a task as completed!");
        println!("  delete <task>      -  delete a task!");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Not enough arguments");
                return;
            }
            let text = args[2..].join(" ");
            let task = Task {
                id: todos_data.next_id,
                text: text,
                is_completed: false,
                created_at: Local::now(),
                completed_at: None,
            };
            todos_data.tasks.push(task);
            todos_data.next_id += 1;
            println!("New Task added!");
        }
        "complete" => {
            if args.len() < 3 {
                println!("Error: Missing Task ID");
                return;
            }
            let id = args[2].parse::<usize>().unwrap();
            if let Some(task) = todos_data.tasks.iter_mut().find(|task| task.id == id) {
                task.is_completed = true;
                task.completed_at = Some(Local::now());
                println!("Task has completed!");
            } else {
                println!("Task with id {} not found", id);
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Error: Missing Task ID");
                return;
            }
            let id = args[2].parse::<usize>().unwrap();
            let length = todos_data.tasks.len();
            todos_data.tasks.retain(|task| task.id != id);
            if todos_data.tasks.len() < length {
                println!("Task has been deleted!")
            } else {
                println!("Task with id {} nto found!", id)
            }
        }
        "list" => {
            if todos_data.tasks.is_empty() {
                println!("No task found!");
            } else {
                for task in &todos_data.tasks {
                    let status = if task.is_completed { "✔ " } else { " " };
                    println!(
                        "[{}] - {}{} (created: {}) {}",
                        task.id,
                        status,
                        task.text,
                        task.created_at.format("%Y-%m-%d %H:%M %A"),
                        if let Some(completed_at) = task.completed_at {
                            format!(" (completed: {})", completed_at.format("%Y-%m-%d %H:%M"))
                        } else {
                            String::new()
                        }
                    );
                }
            }
        }
        _ => println!("Unknown command!"),
    }

    // Always save to the file!
    let json_parsed = serde_json::to_string_pretty(&todos_data).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();
    file.write_all(json_parsed.as_bytes()).unwrap();
}
