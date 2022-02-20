use winapi::um::winuser::{SendMessageW, WM_KEYDOWN};

use crate::windows::WindowHandler;

/// # Safety
/// Send keystroke to the process
pub unsafe fn input(input_handler: &WindowHandler, key: usize) {
    SendMessageW(input_handler.hwnd, WM_KEYDOWN, key, input_handler.lparam);
}
