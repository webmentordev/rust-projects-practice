use rusqlite::Connection;
use std::{env, path::Path};

struct Todo {
    id: usize,
    task: String,
    completed: bool,
    created_at: String,
    completed_at: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db_file = Path::new("/home/ahmer/todos.db").exists();
    if !db_file {
        let conn = Connection::open("/home/ahmer/todos.db").unwrap();
        let sql_query = "
        CREATE TABLE IF NOT EXISTS todos(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            completed BOOLEAN DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            completed_at TIMESTAMP NULL
        )";
        match conn.execute(sql_query, []) {
            Ok(_stmt) => println!("🚀 Database has been created!"),
            Err(e) => println!("Error creating new DB: {e}"),
        }
        return;
    }

    let conn = Connection::open("/home/ahmer/todos.db").unwrap();
    if args.len() == 1 {
        let select_sql_query =
            "SELECT id, task, completed, created_at, completed_at from todos ORDER BY id DESC";
        let mut sql = conn.prepare(select_sql_query).unwrap();
        let todos = sql
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    completed: row.get(2)?,
                    created_at: row.get(3)?,
                    completed_at: row.get(4)?,
                })
            })
            .unwrap();
        for todo in todos {
            match todo {
                Ok(item) => {
                    let status = if item.completed { "✔" } else { " " };
                    print!(
                        "[{}] {} {} (created: {})",
                        item.id, status, item.task, item.created_at
                    );
                    if let Some(completed_at) = item.completed_at {
                        println!(" (completed: {})", completed_at);
                    } else {
                        println!();
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                let task = args[2..].join(" ");
                conn.execute("INSERT INTO todos (task) VALUES (?1)", [task])
                    .unwrap();
                println!("✅ Task has been added!");
            }
            "delete" => {
                if args.len() < 3 {
                    println!("❌ Please provide a task ID to delete");
                    return;
                }
                let id: usize = match args[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("❌ Invalid ID format");
                        return;
                    }
                };
                let rows_affected = conn
                    .execute("DELETE FROM todos WHERE id = ?1", [id])
                    .unwrap();
                if rows_affected > 0 {
                    println!("🗑️ Task deleted successfully!");
                } else {
                    println!("❌ No task found with ID {}", id);
                }
            }
            "complete" => {
                if args.len() < 3 {
                    println!("❌ Please provide a task ID to complete");
                    return;
                }
                let id: usize = match args[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("❌ Invalid ID format");
                        return;
                    }
                };
                let rows_affected = conn
            .execute(
                "UPDATE todos SET completed = 1, completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
                [id],
            )
            .unwrap();
                if rows_affected > 0 {
                    println!("✅ Task marked as completed!");
                } else {
                    println!("❌ No task found with ID {}", id);
                }
            }
            _ => {
                println!("Unknown Command. Please use add, delete, complete.")
            }
        }
    }
}
