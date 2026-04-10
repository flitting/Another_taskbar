# Another Taskbar

`another_taskbar` is a Rust task manager with two interfaces built on the same task data:

- A CLI for quick entry, scripting, and batch changes
- A desktop GUI built with `iced` for browsing, editing, filtering, and theming

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

## Build

```bash
cargo build
cargo build --release
```

## Run

GUI mode is the default:

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

- Task tree browsing with expand/collapse
- Detail and create popups for editing tasks
- Inline editing for task name, description, state, dates, tags, urgency, importance, and pin status
- Search across task names and descriptions
- Filters for tags, urgency, importance, state, and pinned status
- Theme switching from built-in or custom TOML theme files
- Save As / Load file actions from the settings popup
- Docked detail panels or floating popup windows
- Undo for the most recent saved change

GUI-related files:

- `src/gui/` for app state, views, theming, and settings
- `settings.toml` for GUI preferences
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
  gui/          GUI state, views, theming, and settings
  input_parse/  CLI command parsing and prompt helpers
  tasks/        Task model, filtering, and manager logic
  files.rs      JSON persistence and task statistics helpers
  cli_display.rs
  lib.rs
  main.rs
```

## Check

```bash
cargo check
cargo test
```
