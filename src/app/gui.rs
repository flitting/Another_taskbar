use crate::gui;

/// Run the taskbar application in GUI mode.
/// Loads the taskbar and launches an iced window with the task manager.
pub fn run_gui() {
    match gui::run_gui_app() {
        Ok(_) => println!("GUI closed successfully"),
        Err(e) => eprintln!("GUI error: {}", e),
    }
}
