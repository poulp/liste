extern crate ncurses;
extern crate rusqlite;
extern crate rss;

use self::rss::Channel;
use self::rusqlite::Connection;

use controllers::Controller;
use controllers::statusbar::ControllerStatusBar;
use controllers::display::MainDisplayControllers;
use database::get_subscriptions;
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
        // TODO non blocking ui
        self.status_bar.draw_text(String::from("sync !"));
        let subscriptions = get_subscriptions(self.db_connection);
        for subscription in &subscriptions.subscriptions {
            // Download feeds
            let channel_opt = Channel::from_url(subscription.url.as_ref());
            match channel_opt {
                Ok(channel) => {
                    self.db_connection.execute(
                        "UPDATE subscription SET title = ? WHERE subscription_id = ?",
                        &[&channel.title(), &subscription.id]
                    );
                    /* Fetch feeds */
                    for item in channel.items() {
                        /* Save feed in db */
                        self.db_connection.execute(
                            "INSERT INTO feed (title, description, subscription_id) VALUES (?, ?, ?)",
                            &[&item.title(), &item.description(), &subscription.id]
                        ).unwrap();
                    }
                    self.status_bar.draw_text(String::from("ok !"));
                },
                Err(error) => {
                    self.status_bar.draw_text(String::from("error !"));
                }
            }
        }
        self.main_display.after_synchronize();
    }
}
