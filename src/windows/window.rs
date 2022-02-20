use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

use crate::character::{Choice, Player};
use crate::telemetry::Logger;
use crate::time::initial_wait;
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        windef::HWND,
    },
    um::winuser::{EnumWindows, GetWindowTextW},
};

// Required window name
const LOST_ARK: &str = "LOST ARK";

pub struct WindowHandler {
    pub hwnd: HWND,
    pub lparam: LPARAM,
}

impl WindowHandler {
    pub fn new(hwnd: HWND, lparam: LPARAM) -> Self {
        Self { hwnd, lparam }
    }
}

pub struct Window;

impl Window {
    /// # Safety
    pub unsafe fn get_window_and_do_stuff(&self) {
        // Go through all open windows
        EnumWindows(Some(enum_proc), (&LOST_ARK as *const &str) as LPARAM);
    }

    fn windows_logger() -> Logger {
        Logger
    }
}

/// Check if an OS window has the required name
unsafe extern "system" fn enum_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let logger = Window::windows_logger();

    // Buffer to save OS window name
    let mut buf = [0u16; 1024];

    // Check if OS window exists
    if GetWindowTextW(hwnd, &mut buf[0], 1024) > 0 {
        // Get OS window name
        let os_window_name = decode(&buf);

        // Get required window name
        let required_window_name = *(l_param as *const &str) as &str;

        // Check if OS window name contains required window name
        if os_window_name.contains(required_window_name) {
            logger.info("PROCESO DEL LOST ARK RECUPERADO !!!");

            let player = Player::new(WindowHandler::new(hwnd, l_param), logger);
            let mut choice = Choice::new(player, logger);
            let mut rng = rand::thread_rng();

            initial_wait(&logger);

            choice.random_choice(&mut rng);
        }
    }

    // Next OS window from EnumWindows function
    TRUE
}

/// Decode OS window name
fn decode(source: &[u16]) -> String {
    // Change from utf16 to utf8
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
