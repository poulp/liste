extern crate ncurses;

use controller::Controller;
use controller::ControllerStatusBar;
use controller::ControllerSubscriptions;
use controller::ControllerFeeds;
use settings::Settings;

pub struct Screen{
    controllers: Vec<Box<Controller>>,
    status_bar: ControllerStatusBar
}

impl Screen {
    pub fn new(settings: &Settings) -> Screen {
        Screen {
            controllers: vec![
                Box::new(ControllerSubscriptions::new(&settings))
            ],
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
                self.controllers.clear();
                self.controllers.push(
                    Box::new(
                        ControllerFeeds::new(&settings)
                    )
                )
            },
            113 => {
                return true;
            }, // 'q' -> quit
            _ => {} // do nothing
        }
        return false;
    }

    pub fn on_init(&mut self) {
        for controller in self.controllers.iter_mut() {
            controller.on_init();
        }
        self.status_bar.on_init();
    }

    fn on_key_down(&mut self) {
        for controller in self.controllers.iter_mut() {
            controller.on_key_down();
        }
        self.status_bar.on_key_down();
    }

    fn on_key_up(&mut self) {
        for controller in self.controllers.iter_mut() {
            controller.on_key_up();
        }
        self.status_bar.on_key_up();
    }
}