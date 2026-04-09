# CLI Taskbar - Task Management Application

A lightweight, hierarchical task management tool built in Rust with both CLI and GUI interfaces.

## Overview

CLI Taskbar is a practice project that provides a simple yet powerful way to manage tasks with support for:
- Hierarchical task structures (tasks with subtasks)
- Task states (Todo, In Progress, Blocked, Completed, Archived)
- Task pinning and priority management
- Multiple interface modes (CLI and GUI)
- Persistent JSON storage

## Features

### Core Features
- ✅ Create, read, update, and delete tasks
- ✅ Organize tasks hierarchically with subtasks
- ✅ Mark tasks with different states
- ✅ Pin tasks for priority ordering
- ✅ Track creation/update timestamps
- ✅ Persistent storage in JSON format
- ✅ Auto-sorting with pinned tasks first

### CLI Mode Features
- Interactive command-line interface with auto-completion
- Rich task display with formatting
- Batch operations and statistics
- Backup functionality before clearing

### GUI Mode Features (NEW!)
- 🎨 Beautiful graphical interface built with iced
- 📋 Hierarchical task visualization with expand/collapse
- 🎯 Click to view detailed task information
- 🔄 One-click task state management
- 📌 Visual indicators for pinned tasks
- 🎨 Color-coded states for quick recognition

## Installation & Building

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Linux/macOS/Windows with X11 or Wayland support (for GUI)

### Build
```bash
# Clone or navigate to the project
cd cli_taskbar

# Build in debug mode
cargo build

# Build optimized release
cargo build --release
```

### Run
```bash
# Run in CLI mode (default)
cargo run

# Run with explicit CLI mode
cargo run -- --cli

# Run in GUI mode
cargo run -- --gui

# Release builds (faster)
cargo run --release -- --gui
```

## CLI Mode Usage

### Starting the CLI
```bash
cargo run -- --cli
```

### Available Commands

#### Help
```
help                      Show all available commands
```

#### Task Management
```
add [parent_id] [name]    Add a new task (interactive)
update <id>               Update a task interactively
delete <id>               Delete a task by ID
list                      Display all tasks
show <id>                 Show detailed task information
```

#### Information
```
stats                     Display task statistics
```

#### File Operations
```
save                      Save tasks to taskbar.json
load                      Load tasks from taskbar.json
clear                     Clear all tasks (creates backup)
```

#### Exit
```
exit, quit                Save and exit the application
```

### Example CLI Session

```bash
$ cargo run -- --cli
> add 0 Make love
Enter description (or press Enter to skip):
> shower
Enter due date (format: YYYY-MM-DD HH:MM:SS or press Enter to skip):
> Enter urgency (Low/High, default: Low):
High
Enter importance (Low/High, default: Low):
High
Pin this task? (y/n, default: n):
y
Task added with ID: 2

> add 2 shower
Task added with ID: 4

> list
Tasks:
└─ make love (Todo) [PINNED]
   ├─ shower (Todo)
   └─ got to see kaiyue (Todo)
└─ Travel Dinner (Completed)

> save
Saved to taskbar.json

> exit
Goodbye
```

## GUI Mode Usage

### Starting the GUI
```bash
cargo run -- --gui
# or for optimized version
cargo run --release -- --gui
```

### GUI Features

#### Task List (Left Panel)
- **Browse tasks** in a hierarchical tree structure
- **Expand/collapse subtasks** with ▶/▼ buttons
- **Scroll** through your task list
- **Click any task** to view its details

#### State Management (Color Icons)
- Click the colored circle icon next to any task to change its state:
  - **○ Todo** (Blue) - Not started
  - **◔ In Progress** (Orange) - Currently working
  - **⊗ Blocked** (Red) - Cannot proceed
  - **● Completed** (Green) - Finished
  - **◎ Archived** (Gray) - Old/inactive

#### Detail Panel (Right Panel)
- View complete task information when you click a task
- See task metadata (ID, creation date, update date)
- View task description if available
- Count of subtasks
- Close with ✕ button or click the task again

### GUI Layout

```
┌──────────────────────────────────────────────────────────┐
│ 📋 Task Manager                                           │
├──────────────────────────────────┬──────────────────────┤
│ Task List                        │ Detail Panel         │
│ ─────────────────────────────    │ ──────────────────  │
│ ▶ ○ Make love (PINNED)          │ Task Details        │
│   ▼ ◔ shower                     │ ID: 4               │
│   ▼ ○ got to see kaiyue          │ Name: shower        │
│ ▶ ● Travel Dinner               │ State: Todo         │
│                                  │ Pinned: No          │
│                                  │ Subtasks: 0         │
│                                  │ Created: 2026-04... │
│                                  │ Updated: 2026-04... │
└──────────────────────────────────┴──────────────────────┘
```

### GUI Tips
- GUI doesn't auto-save changes (by design for prototype)
- Use CLI mode to make persistent changes
- Expand/collapse state is temporary (only during session)
- Scroll within both panels independently

For detailed GUI documentation, see [GUI_GUIDE.md](GUI_GUIDE.md)

## File Format

Tasks are stored in `taskbar.json` using JSON format:

```json
{
  "root": {
    "id": 0,
    "name": "",
    "description": "",
    "state": "Completed",
    "urgency": null,
    "importance": null,
    "tags": [],
    "pinned": false,
    "subtasks": [
      {
        "id": 1,
        "name": "Example Task",
        "description": "Task description",
        "state": "Todo",
        "urgency": "High",
        "importance": "High",
        "tags": [],
        "pinned": true,
        "subtasks": [],
        "times": {
          "created_at": "2026-04-09T05:07:08.344266758Z",
          "updated_at": "2026-04-09T05:07:08.344269833Z",
          "due_date": null,
          "completed_at": null
        },
        "layer": 1
      }
    ],
    "times": { ... },
    "layer": 0
  },
  "uni_id": 1
}
```

## Project Structure

```
cli_taskbar/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── app.rs            # CLI/GUI mode selection and main loops
│   ├── tasks.rs          # Task data structures and operations
│   ├── gui.rs            # GPUI graphical interface (NEW!)
│   ├── display.rs        # CLI output formatting
│   ├── files.rs          # File I/O operations
│   ├── input_parse.rs    # CLI input parsing
│   └── main.rs           # Entry point
├── Cargo.toml            # Project manifest
├── Cargo.lock            # Dependency lock file
├── taskbar.json          # Default task storage file
├── README.md             # This file
├── GUI_GUIDE.md          # Detailed GUI documentation
└── CHANGES.md            # Changelog
```

## Dependencies

### Core
- `chrono` - Date/time handling
- `serde` + `serde_json` - Serialization/deserialization
- `rustyline` - CLI input with auto-completion

### GUI
- `iced` - Cross-platform GUI framework

## Task Data Model

Each task contains:
- **id**: Unique identifier
- **name**: Task title
- **description**: Detailed description
- **state**: Current state (Todo, InProgress, Blocked, Completed, Archived)
- **urgency**: Optional (Low, High)
- **importance**: Optional (Low, High)
- **tags**: List of tags
- **pinned**: Priority indicator
- **subtasks**: Nested list of child tasks
- **times**: Created, updated, due date, completed date timestamps
- **layer**: Hierarchy level (0 for root, incremented for subtasks)

## Sorting Rules

Tasks are automatically sorted by:
1. **Pinned status** - Pinned tasks appear first
2. **Due date** - Among pinned tasks, by due date (earliest first)
3. **Update time** - Among unpinned tasks, by last update (newest first)

## Task States

| State | Icon | Color | Usage |
|-------|------|-------|-------|
| Todo | ○ | Blue | New, unstarted tasks |
| InProgress | ◔ | Orange | Currently being worked on |
| Blocked | ⊗ | Red | Cannot proceed (waiting for something) |
| Completed | ● | Green | Finished tasks |
| Archived | ◎ | Gray | Old tasks no longer active |

## Development Notes

This is a practice project created to learn Rust and explore different UI frameworks. It demonstrates:
- Hierarchical data structures
- Serialization/deserialization
- Interactive CLI development
- GUI framework usage
- File I/O operations
- State management

### Future Enhancements
- [ ] Database backend (SQLite)
- [ ] Web interface (Actix-web)
- [ ] Task filtering and search
- [ ] Custom themes
- [ ] Keyboard shortcuts
- [ ] Undo/redo functionality
- [ ] Collaborative features
- [ ] Mobile app

## License

This is a practice project. Feel free to use and modify as needed.

## Getting Help

For GUI-specific help, see [GUI_GUIDE.md](GUI_GUIDE.md)

For CLI commands, use the `help` command in the CLI mode:
```
> help
```

## Troubleshooting

### GUI won't start
- Ensure you have X11 or Wayland display server running
- Try building in release mode for better performance
- Check that taskbar.json exists in the current directory

### CLI commands not working
- Use `help` to see available commands
- Make sure you're in the correct working directory
- Check taskbar.json syntax if loading fails

### Build issues
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`
- Check system dependencies for GUI (might need dev packages)