extern crate ncurses;
extern crate rusqlite;

use self::rusqlite::Connection;

use controllers::Controller;
use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use settings::Settings;

pub struct Screen<'a> {
    main_display: MainDisplayControllers<'a>,
    status_bar: ControllerStatusBar,
    db_connection: &'a Connection
}

impl<'a> Screen<'a> {
    pub fn new(settings: &Settings, db_connection: &'a Connection) -> Screen<'a> {
        Screen {
            main_display: MainDisplayControllers::new(&settings, &db_connection),
            status_bar: ControllerStatusBar::new(&settings),
            db_connection: db_connection
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
            114 => {
                // 'r'
                self.on_key_previous();
            },
            115 => {
                // 's' Sync
                self.synchronize();
            },
            113 => {
                return true;
            }, // 'q' -> quit
            _ => {} // do nothing
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

    fn on_key_previous(&mut self) {
        self.main_display.on_key_previous();
        self.status_bar.on_key_previous();
    }

    fn synchronize(&self) {

    }
}