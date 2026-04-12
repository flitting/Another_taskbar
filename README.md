# Another Taskbar

`another_taskbar` is a Rust task manager with two interfaces built on the same task data:

- A CLI for quick entry, scripting, and batch changes
- A desktop GUI built with `tauri` for browsing and editing tasks

The project is being refactored so the task logic stays reusable across desktop, CLI, and a future Android client.

Tasks are stored in JSON as a nested tree with support for subtasks, state tracking, urgency, importance, tags, pinning, and timestamps.

## Features

- Hierarchical tasks with unlimited nesting
- Task states: `Todo`, `InProgress`, `Blocked`, `Completed`, `Archived`
- Optional urgency and importance values
- Shared tags with quick suggestions across the taskbar
- Pinned tasks sorted ahead of others
- Search and filter support
- Undo for the last saved task change
- JSON save/load support
- Theme files loaded from `themes/*.toml`
- Shared runtime/bootstrap for both CLI and GUI
- Recurring tasks

## Build

```bash
cargo build
cargo build --release
```

## Run

CLI mode:

```bash
another_taskbar --cli
```

GUI mode:

```bash
another_taskbar
another_taskbar --gui
```

## CLI

When `taskbar.json` does not exist, the CLI can create a new task file. Commands can be chained on one line.

### Core commands

```text
add [options]
update <id> [options]
delete <id>
delete all [--yes]
list
show <id>
stats
save [--file FILEPATH]
load [--file FILEPATH]
setting NAME VALUE
filter ...
search "STRING"
search --clear
undo
help [COMMAND]
exit
quit
```

### Examples

```bash
add --name "Ship release" --state inprogress --urgency high --importance high --tags release,docs --pinned
add --parent 3 --name "Write README"
update 3 --state completed --tags release,done
filter --importance high
search "release"
save --file work.json
load --file work.json list stats
```

## GUI

Launch the GUI with:

```bash
another_taskbar --gui
```

The GUI includes:

- Task tree browsing (nested subtasks)
- Add, delete, pin/unpin, and complete actions
- Undo for the most recent saved change
- Sort selection directly in the main toolbar
- Theme switching from built-in or custom TOML theme files

GUI-related files:

- `src/gui/` for Tauri backend commands and GUI settings
- `ui/` for the webview frontend
- `themes/light.toml` and `themes/dark.toml` for bundled themes

## Data Files

- Default task file: `taskbar.json`
- GUI settings: `settings.toml`
- Themes: `themes/*.toml`

Task data is serialized as a `TaskManager` tree rooted at an internal node with `id = 0`.

## Project Layout

```text
src/
  app/          CLI and GUI entry points
  app/runtime.rs
  gui/          Tauri backend and GUI settings
  input_parse/  CLI command parsing and prompt helpers
  tasks/        Task model, filtering, and manager logic
  files.rs      JSON persistence and task statistics helpers
  cli_display.rs
  lib.rs
  main.rs
ui/             Tauri webview frontend
tests/          Integration tests for shared behavior
```

## Notes

- The desktop UI now uses bundled frontend font assets instead of exposing font selection in settings.
- Sort mode lives in the main toolbar, while settings are focused on theme, language, and task font size.
- Architecture notes for Android preparation live in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Check

```bash
cargo check
cargo test
```
