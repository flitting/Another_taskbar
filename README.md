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

## Architecture

`another_taskbar` is organized around a shared application core with thin interface layers.

### Goal

Keep task logic, persistence, recurrence, sorting, and settings independent from any one frontend so the project can keep growing toward:

- desktop GUI with Tauri
- CLI workflows
- a future Android client

### Current Structure

**Shared runtime:**

- `src/app/runtime.rs`
  - initializes app storage
  - resolves the active task file path
  - loads persisted GUI settings
  - loads or creates the task manager
  - exposes shared persistence helpers

**Shared domain:**

- `src/tasks/`
  - task model
  - filtering
  - recurrence updates
  - sorting and drag/move semantics
  - undo snapshots

**Interface layers:**

- `src/app/cli.rs`
  - interactive shell
  - delegates task mutations to shared manager APIs
- `src/gui/tauri_app.rs`
  - exposes Tauri commands
  - serializes app snapshot for the web UI
- `ui/`
  - Tauri frontend presentation only

### Android Migration Preparation

The main direction is to keep Android-specific code out of task logic.

**Recommended next steps:**

1. Introduce an `application` service layer for task use-cases such as create, update, move, delete, sort, and recurrence refresh.
2. Make CLI and GUI call those use-cases instead of touching `TaskManager` directly.
3. Move interface DTOs into a dedicated module so Tauri/web payloads are separate from domain structs.
4. Add more integration tests around shared use-cases before adding an Android client.
5. Choose the Android shell later:
   - Tauri mobile if we want to keep the current web UI path
   - a native Android UI if we want platform-native interaction while reusing the Rust core

### Testing Direction

We now keep broader behavior tests in separate files under `tests/` so refactors can validate shared behavior without being tied to one module file.

## Notes

- The desktop UI now uses bundled frontend font assets instead of exposing font selection in settings.
- Sort mode lives in the main toolbar, while settings are focused on theme, language, and task font size.

## Check

```bash
cargo check
cargo test
```
