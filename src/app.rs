extern crate ncurses;
extern crate rusqlite;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::time::Instant;

use self::rusqlite::Connection;

use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use controllers::sync::ControllerSync;
use controllers::component::Component;
use database::{
    init_database,
    get_channels,
};
use models::channels::ListChannels;
use models::feeds::ListFeeds;
use settings::Settings;

const MS_PER_FRAME: u64 = 20;

pub struct Cache {
    pub channels: ListChannels,
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
        let channels = get_channels(&db_connection);
        let (tx, rx) = channel();
        Cache {
            channels: channels,
            feeds: ListFeeds::new(),
            db_connection: db_connection,
            tx: tx,
            rx: rx
        }
    }
}

impl Cache {
    pub fn refresh(&mut self) {
        let channels = get_channels(&self.db_connection);
        self.channels = channels;
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
            let start = Instant::now();
            /* Get user input (async) */
            let ch = ncurses::getch();
            if self.get_input(ch) {
                break;
            }
            /* Get event */
            let result: Result<String, TryRecvError> = self.cache.rx.try_recv();
            match result {
                Ok(event) => {
                    // TODO find a better way to send event from thread
                    match event.as_ref() {
                        "cdone" => {
                            self.on_channel_synchronize_done();
                        },
                        "done" => {
                            self.on_synchronize_done();
                        },
                        _ => {
                            self.on_channel_synchronize_start(&event);
                        }
                    }
                },
                Err(_) => {}
            }
            let sleep_time = (MS_PER_FRAME / 1000) - start.elapsed().as_secs();
            thread::sleep(Duration::from_secs(sleep_time));
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

    fn on_channel_synchronize_done(&mut self) {
        for component in self.components.iter_mut() {
            component.on_channel_synchronize_done(&mut self.cache);
        }
    }
}
