extern crate ncurses;

use super::Controller;
use super::super::windows::statusbar::WindowStatusBar;
use super::super::settings::Settings;

pub struct ControllerStatusBar {
    window: WindowStatusBar
}

impl ControllerStatusBar {
    pub fn new(settings: &Settings) -> ControllerStatusBar {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();

        let window = WindowStatusBar::new();

        ControllerStatusBar {
            window: window,
        }
    }
}

impl Controller for ControllerStatusBar {

    fn on_init(&mut self) {
        self.window.draw(String::from("status"));
    }

    fn on_key_down(&mut self){}

    fn on_key_up(&mut self){}

    fn on_key_enter(&mut self){}

    fn on_key_previous(&mut self) {}
}

impl ControllerStatusBar {
    pub fn draw_text(&mut self, text: String) {
        self.window.draw(text);

    }
}