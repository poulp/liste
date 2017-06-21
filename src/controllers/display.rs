extern crate ncurses;

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

pub struct MainDisplayControllers {
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
    current_window: String
}

impl MainDisplayControllers {
    pub fn new(settings: &Settings) -> MainDisplayControllers {
        let mut feeds = ListFeeds::new();
        feeds.add_feed(String::from("test"));
        feeds.add_feed(String::from("salut"));
        /* Copy subscriptions from settings */
        let mut subscriptions = settings.subscriptions.to_owned();

        MainDisplayControllers {
            window_subscriptions: WindowList::new(),
            window_feeds: WindowList::new(),
            window_feed: WindowText::new(),
            subscriptions: subscriptions,
            feeds: feeds,
            feed: String::from("--"), // Empty content
            current_window: String::from("subscriptions")
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

impl Controller for MainDisplayControllers {
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