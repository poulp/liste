extern crate ncurses;

use controller::Controller;
use controller::ControllerStatusBar;
use controller::MainDisplayControllers;
use settings::Settings;

pub struct Screen{
    main_display: MainDisplayControllers,
    status_bar: ControllerStatusBar
}

impl Screen {
    pub fn new(settings: &Settings) -> Screen {
        Screen {
            main_display: MainDisplayControllers::new(&settings),
            status_bar: ControllerStatusBar::new(&settings)
        }
    }

    pub fn get_input(&mut self, ch: i32, settings: &Settings) -> bool {
        /* return true to quit */
        match ch {
            ncurses::KEY_DOWN => {
                self.on_key_down();
            },
            ncurses::KEY_UP => {
                self.on_key_up();
            },
            10 => {
                self.on_key_enter();
            },
            113 => {
                return true;
            }, // 'q' -> quit
            _ => {
            } // do nothing
        }
        return false;
    }

    pub fn on_init(&mut self) {
        self.main_display.on_init();
        self.status_bar.on_init();
    }

    fn on_key_down(&mut self) {
        self.main_display.on_key_down();
        self.status_bar.on_key_down();
    }

    fn on_key_up(&mut self) {
        self.main_display.on_key_up();
        self.status_bar.on_key_up();
    }

    fn on_key_enter(&mut self) {
        self.main_display.on_key_enter();
        self.status_bar.on_key_enter();
    }
}