extern crate ncurses;

use super::Controller;
use super::super::window::WindowStatusBar;
use super::super::settings::Settings;

pub struct ControllerStatusBar {
    window: WindowStatusBar
}

impl ControllerStatusBar {
    pub fn new(settings: &Settings) -> ControllerStatusBar {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();

        let mut window = WindowStatusBar::new();

        ControllerStatusBar {
            window: window,
        }
    }
}

impl Controller for ControllerStatusBar {

    fn on_init(&mut self) {
        self.window.draw();
    }

    fn on_key_down(&mut self){}

    fn on_key_up(&mut self){}

    fn on_key_enter(&mut self){}

    fn on_key_previous(&mut self) {}
}