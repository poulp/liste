use windows::statusbar::WindowStatusBar;

pub struct ControllerStatusBar {
    window: WindowStatusBar
}

impl ControllerStatusBar {
    pub fn new() -> ControllerStatusBar {
        let window = WindowStatusBar::new();
        ControllerStatusBar {
            window: window,
        }
    }

    pub fn on_init(&mut self) {
        self.window.draw(String::from("status"));
    }

    pub fn draw_text(&mut self, text: String) {
        self.window.draw(text);
    }
}
