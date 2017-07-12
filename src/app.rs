extern crate ncurses;
extern crate rusqlite;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use self::rusqlite::Connection;

use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use controllers::sync::ControllerSync;
use controllers::component::Component;
use database::{
    init_database,
    get_subscriptions,
};
use models::subscriptions::ListSubscriptions;
use models::feeds::ListFeeds;
use settings::Settings;

const MS_PER_FRAME: u64 = 30;

pub struct Cache {
    pub subscriptions: ListSubscriptions,
    pub feeds: ListFeeds,
    pub db_connection: Connection,

    pub tx: Sender<String>,
    pub rx: Receiver<String>
}

impl Cache {
    pub fn new(settings: &Settings) -> Cache {
        /* Open database */
        let db_connection = Connection::open("base.db").unwrap();
        /* Create tables */
        init_database(&db_connection, &settings);
        let subscriptions = get_subscriptions(&db_connection);
        let (tx, rx) = channel();
        Cache {
            subscriptions: subscriptions,
            feeds: ListFeeds::new(),
            db_connection: db_connection,
            tx: tx,
            rx: rx
        }
    }
}

impl Cache {
    pub fn refresh(&mut self) {
        let subscriptions = get_subscriptions(&self.db_connection);
        self.subscriptions = subscriptions;
    }
}

pub struct Application<> {
    components: Vec<Box<Component>>,
    cache: Cache,
}

impl<> Application<> {
    pub fn new(settings: &Settings) -> Application {
        let mut components: Vec<Box<Component>> = Vec::new();
        components.push(Box::new(MainDisplayControllers::new()));
        components.push(Box::new(ControllerStatusBar::new()));
        components.push(Box::new(ControllerSync::new()));

        Application {
            components: components,
            cache: Cache::new(settings),
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
            let result: Result<String, TryRecvError> = self.cache.rx.try_recv();
            match result {
                Ok(event) => {
                    if event.eq("done") {
                        // TODO find a better way to send event from thread
                        self.on_synchronize_done();
                    } else {
                        self.on_channel_synchronize_start(&event);
                    }
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
                self.on_synchronize_start();
            },
            113 => {
                // 'q'
                return true;
            },
            _ => {} // do nothing
        }
        return false;
    }

    fn on_init(&mut self) {
        for component in self.components.iter_mut() {
            component.on_init(&self.cache);
        }
    }

    fn on_key_down(&mut self) {
        for component in self.components.iter_mut() {
            component.on_key_down(&self.cache);
        }
    }

    fn on_key_up(&mut self) {
        for component in self.components.iter_mut() {
            component.on_key_up(&self.cache);
        }
    }

    fn on_key_enter(&mut self) {
        for component in self.components.iter_mut() {
            component.on_key_enter(&mut self.cache);
        }
    }

    fn on_key_previous(&mut self) {
        for component in self.components.iter_mut() {
            component.on_key_previous(&self.cache);
        }
    }

    fn on_synchronize_start(&mut self) {
        for component in self.components.iter_mut() {
            component.on_synchronize_start(&mut self.cache);
        }
    }

    fn on_synchronize_done(&mut self) {
        /* Reload cache */
        self.cache.refresh();
        for component in self.components.iter_mut() {
            component.on_synchronize_done(&mut self.cache);
        }
    }

    fn on_channel_synchronize_start(&mut self, event: &str) {
        for component in self.components.iter_mut() {
            component.on_channel_synchronize_start(&mut self.cache, event);
        }
    }
}
