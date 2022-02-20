use crate::telemetry::Logger;
use crate::windows::Window;

struct App {
    window: Window,
    logger: Logger,
}

impl App {
    fn new(window: Window, logger: Logger) -> Self {
        Self { window, logger }
    }
}

/// # Safety
pub unsafe fn run() {
    let app = App::new(Window, Logger);

    app.logger.welcome();

    app.window.get_window_and_do_stuff();

    app.logger.exit();
}
