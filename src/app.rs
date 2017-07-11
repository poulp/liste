extern crate ncurses;
extern crate rusqlite;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use self::rusqlite::Connection;

use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use controllers::sync::ControllerSync;

use database::{
    get_subscriptions,
    create_feed
};
use settings::Settings;

const MS_PER_FRAME: u64 = 40;

pub struct Application<'a> {
    main_display: MainDisplayControllers<'a>,
    status_bar: ControllerStatusBar,
    ctrl_sync: ControllerSync,

    tx: Sender<String>,
    rx: Receiver<String>
}

impl<'a> Application<'a> {
    pub fn new(settings: &Settings, db_connection: &'a Connection) -> Application<'a> {
        let (tx, rx) = channel();
        Application {
            main_display: MainDisplayControllers::new(&db_connection),
            status_bar: ControllerStatusBar::new(),
            ctrl_sync: ControllerSync::new(),
            tx: tx,
            rx: rx
        }
    }

    pub fn main_loop(&mut self) {
        ncurses::refresh();
        self.on_init();

        loop {
            /* Get user input (async) */
            let ch = ncurses::getch();
            if self.get_input(ch) {
                break;
            }
            /* Get event */
            let result: Result<String, TryRecvError> = self.rx.try_recv();
            match result {
                Ok(event) => {
                    self.status_bar.draw_text(event);
                    self.main_display.refresh();
                },
                Err(_) => {}
            }
            thread::sleep(Duration::from_millis(MS_PER_FRAME));
        }
    }

    pub fn get_input(&mut self, ch: i32) -> bool {
        /* Return true to quit the application */
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
                // 's'
                self.synchronize();
            },
            113 => {
                // 'q'
                return true;
            },
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
    }

    fn on_key_up(&mut self) {
        self.main_display.on_key_up();
    }

    fn on_key_enter(&mut self) {
        self.main_display.on_key_enter();
    }

    fn on_key_previous(&mut self) {
        self.main_display.on_key_previous();
    }

    fn synchronize(&mut self) {
        self.ctrl_sync.synchronize(self.tx.clone());
    }
}
