use std::env;

use another_taskbar::app;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command-line arguments (skip program name at index 0)
    let mode = app::parse_mode(&args);

    #[cfg(target_os = "windows")]
    if mode == "gui" {
        hide_console_window();
    }

    match mode {
        "cli" => app::run_cli(),
        "gui" => app::run_gui(),
        _ => unreachable!(),
    }
}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetConsoleWindow() -> *mut core::ffi::c_void;
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn ShowWindow(h_wnd: *mut core::ffi::c_void, n_cmd_show: i32) -> i32;
    }

    const SW_HIDE: i32 = 0;

    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_null() {
            let _ = ShowWindow(console_window, SW_HIDE);
        }
    }
}
