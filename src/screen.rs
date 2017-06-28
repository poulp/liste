extern crate ncurses;
extern crate rusqlite;
extern crate rss;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use self::rss::Channel;
use self::rusqlite::Connection;

use controllers::Controller;
use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use database::{
    get_subscriptions,
    create_feed
};
use settings::Settings;

const MS_PER_FRAME: u64 = 40;

pub struct Screen<'a> {
    main_display: MainDisplayControllers<'a>,
    status_bar: ControllerStatusBar,
    db_connection: &'a Connection,

    tx: Sender<String>,
    rx: Receiver<String>
}

impl<'a> Screen<'a> {
    pub fn new(settings: &Settings, db_connection: &'a Connection) -> Screen<'a> {
        let (tx, rx) = channel();
        Screen {
            main_display: MainDisplayControllers::new(&settings, &db_connection),
            status_bar: ControllerStatusBar::new(&settings),
            db_connection: db_connection,
            tx: tx,
            rx: rx
        }
    }

    pub fn main_loop(&mut self, settings: &Settings) {
        ncurses::refresh();
        self.on_init();

        loop {
            /* Get user input (async) */
            let ch = ncurses::getch();
            if self.get_input(ch, settings) {
                break;
            }
            /* Get event */
            let result: Result<String, TryRecvError> = self.rx.try_recv();
            match result {
                Ok(event) => {
                    self.status_bar.draw_text(event);
                },
                Err(_) => {}
            }
            thread::sleep(Duration::from_millis(MS_PER_FRAME));
        }
    }

    pub fn get_input(&mut self, ch: i32, settings: &Settings) -> bool {
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

    fn synchronize(&mut self) {
        let tx_sync = self.tx.clone();
        thread::spawn(move || {
            let db_conn = Connection::open("base.db").unwrap();
            let subscriptions = get_subscriptions(&db_conn);
            let len_sub = subscriptions.subscriptions.len();

            for (index, subscription) in subscriptions.subscriptions.iter().enumerate() {
                tx_sync.send(
                    format!("Download channels : {}/{}", index, len_sub)
                ).unwrap();
                // Download feeds
                let channel_opt = Channel::from_url(subscription.url.as_ref());
                match channel_opt {
                    Ok(channel) => {
                        db_conn.execute(
                            "UPDATE subscription SET title = ? WHERE subscription_id = ?",
                            &[&channel.title(), &subscription.id]
                        );
                        /* Fetch feeds */
                        for item in channel.items() {
                            /* Save feed in db */
                            create_feed(
                                &db_conn,
                                item.title().unwrap(),
                                item.description().unwrap(),
                                subscription.id
                            )
                        }

                    },
                    Err(error) => {
                        //self.status_bar.draw_text(String::from("error !"));
                    }
                }
                tx_sync.send(
                    format!("Download channels : {len_sub}/{len_sub} Done !", len_sub=len_sub)
                ).unwrap();
            }
            //self.main_display.after_synchronize();

        });
    }
}
