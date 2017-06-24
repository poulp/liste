extern crate ncurses;
extern crate rusqlite;

use self::rusqlite::Connection;

use super::super::database::get_subscriptions;
use super::Controller;
use super::super::windows::list::WindowList;
use super::super::windows::text::WindowText;
use super::super::models::subscriptions::{
    Subscription,
    ListSubscriptions
};
use super::super::models::feeds::{
    Feed,
    ListFeeds
};
use super::super::settings::Settings;

pub struct MainDisplayControllers<'a> {
    window_subscriptions: WindowList,
    window_feeds: WindowList,
    window_feed: WindowText,

    subscriptions: ListSubscriptions,
    feeds: ListFeeds,
    feed: String,

    /* possible values :
     *  - subscriptions
     *  - feeds
     *  - read
     */
    // TODO find a better way
    current_window: String,
    db_connection: &'a Connection
}

impl<'a> MainDisplayControllers<'a> {
    pub fn new(settings: &Settings, db_connection: &'a Connection) -> MainDisplayControllers<'a> {
        /* Copy subscriptions from settings
         * TODO get subscriptions from database
         */
        //let mut subscriptions = settings.subscriptions.to_owned();

        let mut subscriptions = get_subscriptions(db_connection);

        MainDisplayControllers {
            window_subscriptions: WindowList::new(),
            window_feeds: WindowList::new(),
            window_feed: WindowText::new(),
            subscriptions: subscriptions,
            feeds: ListFeeds::new(),
            feed: String::from("--"), // Empty content
            current_window: String::from("subscriptions"),
            db_connection: db_connection

        }
    }

    fn draw(&mut self) {
        self.window_subscriptions.clear();
        match self.current_window.as_ref() {
            "subscriptions" => {
                self.window_subscriptions.draw(&self.subscriptions.subscriptions);
            },
            "feeds" => {
                self.window_feeds.draw(&self.feeds.feeds);
            },
            "read" => {
                self.window_feed.draw(self.feed.as_ref())
            },
            _ => {}
        }
    }

    fn clear_windows(&mut self, to: String) {
        self.window_subscriptions.clear();
        self.window_feeds.clear();
        self.window_feed.clear();
    }
}

impl<'a> Controller for MainDisplayControllers<'a> {
    fn on_init(&mut self) {
        self.window_subscriptions.draw(&self.subscriptions.subscriptions);
    }

    fn on_key_down(&mut self) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "subscriptions" => {
                if !self.subscriptions.subscriptions.is_empty() {
                    if self.window_subscriptions.active_item + 1 < self.subscriptions.subscriptions.len() as i32 {
                        self.window_subscriptions.active_item += 1;
                        self.draw();
                    }
                }
            },
            "feeds" => {
                if !self.feeds.feeds.is_empty() {
                    if self.window_feeds.active_item + 1 < self.feeds.feeds.len() as i32 {
                        self.window_feeds.active_item += 1;
                        self.draw();
                    }
                }
            },
            "read" => {
                // TODO scroll down
            },
            _ => {}
        }
    }

    fn on_key_up(&mut self) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "subscriptions" => {
                if !self.subscriptions.subscriptions.is_empty() {
                    if self.window_subscriptions.active_item - 1 >= 0 {
                        self.window_subscriptions.active_item -= 1;
                        self.draw();
                    }
                }
            },
            "feeds" => {
                if !self.feeds.feeds.is_empty() {
                    if self.window_feeds.active_item - 1 >= 0 {
                        self.window_feeds.active_item -= 1;
                        self.draw();
                    }
                }
            },
            "read" => {
                // TODO scroll up
            },
            _ => {}
        }
    }

    fn on_key_enter(&mut self) {
        match self.current_window.as_ref() {
            "subscriptions" => {
                /* Clear feeds */
                self.feeds.clear();
                /* Load feeds */
                let mut statement = self.db_connection.prepare("SELECT content FROM feed").unwrap();
                let rows = statement.query_map(&[], |row| -> String {row.get(0)}).unwrap();
                for row in rows {
                    let feed_name = row.unwrap();
                    self.feeds.add_feed(feed_name);
                }
                self.current_window = String::from("feeds");
                self.draw();
            },
            "feeds" => {
                self.current_window = String::from("read");
                self.draw();
            },
            "read" => {
                // nothing happen here
            },
            _ => {}
        }
    }

    fn on_key_previous(&mut self) {
        match self.current_window.as_ref() {
            "subscriptions" => {
                // nothing here
            },
            "feeds" => {
                self.current_window = String::from("subscriptions");
                self.draw();
            },
            "read" => {
                self.current_window = String::from("feeds");
                self.draw();
            },
            _ => {}
        }
    }
}