# System design

## Folder structure:
```
todo-cli/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── task.rs         # Task struct and methods
│   ├── storage.rs      # JSON file I/O
│   └── commands.rs     # Command implementations
├── tests/
│   └── integration_tests.rs
└── Cargo.toml
```

## Data model (main entities):
1. **Task:** Single todo item with status tracking
   - Fields: id (u32), description (String), created_at (DateTime), completed_at (Option<DateTime>), is_done (bool)
2. **TaskList:** Container for all tasks with persistence methods
   - Fields: tasks (Vec<Task>), file_path (PathBuf)

## Flow (user journey):
1. User runs `todo add "Buy groceries"`
2. App loads existing tasks from ~/.todo.json
3. Creates new Task with auto-incrementing ID
4. Appends to TaskList
5. Saves TaskList back to JSON file
6. Displays confirmation message

## Critical technical decisions:
- Data persistence: Single JSON file at ~/.todo.json for simplicity
- ID generation: Auto-incrementing integer based on max existing ID + 1
- Error handling: Result<> types throughout, user-friendly error messages
- File locking: Not implemented in MVP (single-user assumption)

---
✅ Done when: Folder structure + data model + user flow documented
