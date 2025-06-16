SQLite DB will be stored in /tmp/todos.db

Make sure to assign +x to the executable
```
To add Task
cargo run -- add This is example task
cargo run -- delete id-of-task
cargo run -- complete id-of-task
cargo run
(Simple cargo run will print tasks if exist)
```

Run this as 'todos' command
```
sudo cp target/release/sqlite_todo /usr/local/bin/todos
```