# Architecture Notes

`another_taskbar` is now organized around a shared application core with thin interface layers.

## Goal

Keep task logic, persistence, recurrence, sorting, and settings independent from any one frontend so the project can keep growing toward:

- desktop GUI with Tauri
- CLI workflows
- a future Android client

## Current structure

Shared runtime:

- `src/app/runtime.rs`
  - initializes app storage
  - resolves the active task file path
  - loads persisted GUI settings
  - loads or creates the task manager
  - exposes shared persistence helpers

Shared domain:

- `src/tasks/`
  - task model
  - filtering
  - recurrence updates
  - sorting and drag/move semantics
  - undo snapshots

Interface layers:

- `src/app/cli.rs`
  - interactive shell
  - delegates task mutations to shared manager APIs
- `src/gui/tauri_app.rs`
  - exposes Tauri commands
  - serializes app snapshot for the web UI
- `ui/`
  - Tauri frontend presentation only

## Android migration preparation

The main direction is to keep Android-specific code out of task logic.

Recommended next steps:

1. Introduce an `application` service layer for task use-cases such as create, update, move, delete, sort, and recurrence refresh.
2. Make CLI and GUI call those use-cases instead of touching `TaskManager` directly.
3. Move interface DTOs into a dedicated module so Tauri/web payloads are separate from domain structs.
4. Add more integration tests around shared use-cases before adding an Android client.
5. Choose the Android shell later:
   - Tauri mobile if we want to keep the current web UI path
   - a native Android UI if we want platform-native interaction while reusing the Rust core

## Testing direction

We now keep broader behavior tests in separate files under `tests/` so refactors can validate shared behavior without being tied to one module file.
