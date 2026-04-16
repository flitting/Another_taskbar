#[cfg(target_os = "windows")]
pub fn hide_console_for_gui_if_standalone() {
    use windows_sys::Win32::System::Console::{GetConsoleProcessList, GetConsoleWindow};
    use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};

    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd == 0 {
            return;
        }

        let mut process_ids = [0u32; 2];
        let count = GetConsoleProcessList(process_ids.as_mut_ptr(), process_ids.len() as u32);

        // If the console is only attached to this process, it was likely spawned
        // just for this app. Leave shared parent terminals alone.
        if count <= 1 {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn hide_console_for_gui_if_standalone() {
    // No-op for non-Windows platforms
}
