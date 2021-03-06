extern crate ncurses;
extern crate rusqlite;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
// use std::time::Instant;

use self::rusqlite::Connection;

use components::statusbar::ComponentStatusBar;
use components::core::ComponentCore;
use components::sync::ComponentSync;
use components::component::Component;
use database::{
    init_database,
    get_channels,
};
use models::channels::Channel;
use models::items::Item;
use settings::Settings;

// const MS_PER_FRAME: u64 = 60;

pub struct Cache {
    /* Store data used by components */
    pub channels: Vec<Channel>,
    pub items: Vec<Item>,
    pub db_connection: Connection,

    pub tx: Sender<String>,
    pub rx: Receiver<String>,
    pub db_lock: Arc<Mutex<i32>>
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
            channels,
            items: vec![],
            db_connection,
            tx,
            rx,
            db_lock: Arc::new(Mutex::new(0))
        }
    }
}

impl Cache {
    pub fn refresh(&mut self) {
        let channels = get_channels(&self.db_connection);
        self.channels = channels;
    }

    pub fn set_item_as_read(&mut self, index: i32) {
        let item = self.items.get_mut(
                        index as usize).unwrap();
        item.set_as_read(&self.db_connection, true);
    }
}

pub struct Application<> {
    components: Vec<Box<Component>>,
    cache: Cache,
}

impl<> Application<> {
    pub fn new(settings: &Settings) -> Application {
        let mut components: Vec<Box<Component>> = Vec::new();
        components.push(Box::new(ComponentCore::new()));
        components.push(Box::new(ComponentStatusBar::new()));
        components.push(Box::new(ComponentSync::new()));

        Application {
            components,
            cache: Cache::new(settings),
        }
    }

    pub fn main_loop(&mut self) {
        ncurses::refresh();
        self.on_init();

        loop {
            // let start = Instant::now();
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
                        "done" => {
                            self.on_synchronize_done();
                        }
                        _ => {}
                    }
                },
                Err(_) => {}
            }
            //let sleep_time = (MS_PER_FRAME / 1000) - start.elapsed().as_secs();
            thread::sleep(Duration::from_millis(20));
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
            component.on_key_previous(&mut self.cache);
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
}
