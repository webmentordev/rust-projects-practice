use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
            title: title.to_string(),
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.next_id += 1;
        self.tasks.push(task);
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            task.completed_at = Some(Local::now());
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn delete_task(&mut self, id: usize) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);

        if self.tasks.len() < initial_len {
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn list_tasks(&self, show_completed: bool) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| show_completed || !task.completed)
            .collect()
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }

    fn load_from_file(file_path: &str) -> Result<Self, String> {
        if !Path::new(file_path).exists() {
            return Ok(TodoList::new());
        }

        let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if contents.trim().is_empty() {
            return Ok(TodoList::new());
        }

        serde_json::from_str(&contents).map_err(|e| format!("Failed to deserialize tasks: {}", e))
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  add <task>       - Add a new task");
    println!("  complete <id>    - Mark a task as completed");
    println!("  delete <id>      - Delete a task");
    println!("  list             - List all pending tasks");
    println!("  list-all         - List all tasks (including completed)");
    println!("  help             - Show this help message");
}

fn main() {
    let file_path = "todo.json";
    let mut todo_list = match TodoList::load_from_file(file_path) {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error loading todo list: {}", err);
            TodoList::new()
        }
    };

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Error: Missing task description");
                return;
            }
            let task_title = args[2..].join(" ");
            todo_list.add_task(&task_title);
            println!("Task added: {}", task_title);
        }
        "complete" => {
            if args.len() < 3 {
                println!("Error: Missing task ID");
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => match todo_list.complete_task(id) {
                    Ok(_) => println!("Task {} marked as completed", id),
                    Err(err) => println!("Error: {}", err),
                },
                Err(_) => println!("Error: Task ID must be a number"),
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Error: Missing task ID");
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => match todo_list.delete_task(id) {
                    Ok(_) => println!("Task {} deleted", id),
                    Err(err) => println!("Error: {}", err),
                },
                Err(_) => println!("Error: Task ID must be a number"),
            }
        }
        "list" => {
            let tasks = todo_list.list_tasks(false);
            if tasks.is_empty() {
                println!("No pending tasks!");
            } else {
                println!("Pending Tasks:");
                for task in tasks {
                    println!(
                        "[{}] {} (created: {})",
                        task.id,
                        task.title,
                        task.created_at.format("%Y-%m-%d %H:%M")
                    );
                }
            }
        }
        "list-all" => {
            let tasks = todo_list.list_tasks(true);
            if tasks.is_empty() {
                println!("No tasks!");
            } else {
                println!("All Tasks:");
                for task in tasks {
                    let status = if task.completed { "✓" } else { " " };
                    println!(
                        "[{}] {}{} (created: {}){}",
                        task.id,
                        status,
                        task.title,
                        task.created_at.format("%Y-%m-%d %H:%M"),
                        if let Some(completed_at) = task.completed_at {
                            format!(" (completed: {})", completed_at.format("%Y-%m-%d %H:%M"))
                        } else {
                            String::new()
                        }
                    );
                }
            }
        }
        "help" => {
            print_usage();
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }

    // Save the todo list to file after each operation
    if let Err(err) = todo_list.save_to_file(file_path) {
        eprintln!("Error saving todo list: {}", err);
    }
}
