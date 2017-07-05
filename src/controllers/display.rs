extern crate ncurses;
extern crate rusqlite;

use self::rusqlite::Connection;

use super::super::database::{
    get_subscriptions,
    get_feeds_from_subscription
};
use super::Controller;
use super::super::windows::list::WindowList;
use super::super::windows::text::WindowText;
use super::super::models::subscriptions::ListSubscriptions;
use super::super::models::feeds::ListFeeds;

pub struct MainDisplayControllers<'a> {
    window_subscriptions: WindowList,
    window_feeds: WindowList,
    window_feed: WindowText,

    subscriptions: ListSubscriptions,
    feeds: ListFeeds,

    /* possible values :
     *  - subscriptions
     *  - feeds
     *  - read
     */
    // TODO find a better way
    current_window: String,
    active_subscription_index: i32,
    active_feed_index: i32,
    db_connection: &'a Connection
}

impl<'a> MainDisplayControllers<'a> {
    pub fn new(db_connection: &'a Connection) -> MainDisplayControllers<'a> {
        let subscriptions = get_subscriptions(db_connection);
        let cols_channel = vec![
            (String::from("Unread"), 12),
            (String::from("Channel"), 16),
        ];
        MainDisplayControllers {
            window_subscriptions: WindowList::new(cols_channel),
            window_feeds: WindowList::new(vec![]),
            window_feed: WindowText::new(),
            subscriptions: subscriptions,
            feeds: ListFeeds::new(),
            current_window: String::from("subscriptions"),
            db_connection: db_connection,
            active_subscription_index: 0,
            active_feed_index: 0
        }
    }

    fn draw(&mut self) {
        self.clear_windows();
        match self.current_window.as_ref() {
            "subscriptions" => {
                let cols = self.get_subscriptions_cols();
                self.window_subscriptions.draw(
                    self.active_subscription_index,
                    &cols
                );
            },
            "feeds" => {
                let cols = self.get_feeds_cols();
                self.window_feeds.draw(
                    self.active_feed_index,
                    &cols
                );
            },
            "read" => {
                // TODO work with reference
                let feed = self.feeds.feeds.get(self.active_feed_index as usize).unwrap();
                self.window_feed.draw(feed);
            },
            _ => {}
        }
    }

    fn clear_windows(&mut self) {
        self.window_subscriptions.clear();
        self.window_feeds.clear();
        self.window_feed.clear();
    }

    fn set_next_active_sub_index(&mut self) {
        if !self.subscriptions.subscriptions.is_empty() {
            if self.active_subscription_index + 1 < self.subscriptions.subscriptions.len() as i32 {
                self.active_subscription_index += 1;
                self.draw();
            }
        }
    }

    fn set_previous_active_sub_index(&mut self) {
        if !self.subscriptions.subscriptions.is_empty() {
            if self.active_subscription_index - 1 >= 0 {
                self.active_subscription_index -= 1;
                self.draw();
            }
        }
    }

    fn set_next_active_feed_index(&mut self) {
        if !self.feeds.feeds.is_empty() {
            if self.active_feed_index + 1 < self.feeds.feeds.len() as i32 {
                self.active_feed_index += 1;
                self.draw();
            }
        }
    }

    fn set_previous_active_feed_index(&mut self) {
        if !self.feeds.feeds.is_empty() {
            if self.active_feed_index - 1 >= 0 {
                self.active_feed_index -= 1;
                self.draw();
            }
        }
    }

    fn get_subscriptions_cols(&self) -> Vec<Vec<String>> {
        let mut list_cols: Vec<Vec<String>> = vec![];
        for subscription in &self.subscriptions.subscriptions {
            list_cols.push(vec![
                subscription.total_feed_unread.to_string(), // TODO check db
                String::from(subscription.title()) // TODO use ref ?
            ]);
        }
        list_cols
    }

    fn get_feeds_cols(&self) -> Vec<Vec<String>> {
        let mut list_feeds: Vec<Vec<String>> = vec![];
        for feed in &self.feeds.feeds {
            list_feeds.push(vec![String::from(feed.title.as_ref())]);
        }
        list_feeds
    }
}

impl<'a> Controller for MainDisplayControllers<'a> {

    fn on_init(&mut self) {
        self.draw();
    }

    fn on_key_down(&mut self) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "subscriptions" => {
                self.set_next_active_sub_index();
            },
            "feeds" => {
                self.set_next_active_feed_index();
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
                self.set_previous_active_sub_index();
            },
            "feeds" => {
                self.set_previous_active_feed_index();
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
                /* Get active subscription id */
                // TODO improve here
                let id_sub = self.subscriptions.subscriptions
                    .get(self.active_subscription_index as usize)
                    .unwrap()
                    .id;
                /* Fetch feeds from db */
                self.feeds = get_feeds_from_subscription(
                    self.db_connection,
                    id_sub
                );
                /* Init active item index */
                self.active_feed_index = 0;
                /* Load the feeds screen */
                self.current_window = String::from("feeds");
                self.draw();
            },
            "feeds" => {
                if !self.feeds.feeds.is_empty() {
                    self.current_window = String::from("read");
                    self.draw();
                }
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
