extern crate ncurses;

use super::super::database::{
    get_feeds_from_subscription
};
use windows::list::WindowList;
use windows::text::WindowText;
use app::Cache;
use controllers::component::Component;

pub struct MainDisplayControllers {
    window_subscriptions: WindowList,
    window_feeds: WindowList,
    window_feed: WindowText,

    /* possible values :
     *  - subscriptions
     *  - feeds
     *  - read
     */
    // TODO find a better way
    // TODO state pattern
    current_window: String,
    active_subscription_index: i32,
    active_feed_index: i32,
}

impl MainDisplayControllers {
    pub fn new() -> MainDisplayControllers {
        let cols_channel = vec![
            (String::from("Unread"), 12),
            (String::from("Channel"), 16),
        ];
        MainDisplayControllers {
            window_subscriptions: WindowList::new(cols_channel),
            window_feeds: WindowList::new(vec![]),
            window_feed: WindowText::new(),
            current_window: String::from("subscriptions"),
            active_subscription_index: 0,
            active_feed_index: 0
        }
    }

    fn draw(&mut self, cache: &Cache) {
        self.clear_windows();
        match self.current_window.as_ref() {
            "subscriptions" => {
                let cols = self.get_subscriptions_cols(cache);
                self.window_subscriptions.draw(
                    self.active_subscription_index,
                    &cols
                );
            },
            "feeds" => {
                let cols = self.get_feeds_cols(cache);
                self.window_feeds.draw(
                    self.active_feed_index,
                    &cols
                );
            },
            "read" => {
                // TODO work with reference
                let feed = cache.feeds.feeds.get(self.active_feed_index as usize).unwrap();
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

    fn set_next_active_sub_index(&mut self, cache: &Cache) {
        if !cache.subscriptions.subscriptions.is_empty() {
            if self.active_subscription_index + 1 < cache.subscriptions.subscriptions.len() as i32 {
                self.active_subscription_index += 1;
                self.draw(cache);
            }
        }
    }

    fn set_previous_active_sub_index(&mut self, cache: &Cache) {
        if !cache.subscriptions.subscriptions.is_empty() {
            if self.active_subscription_index - 1 >= 0 {
                self.active_subscription_index -= 1;
                self.draw(cache);
            }
        }
    }

    fn set_next_active_feed_index(&mut self, cache: &Cache) {
        if !cache.feeds.feeds.is_empty() {
            if self.active_feed_index + 1 < cache.feeds.feeds.len() as i32 {
                self.active_feed_index += 1;
                self.draw(cache);
            }
        }
    }

    fn set_previous_active_feed_index(&mut self, cache: &Cache) {
        if !cache.feeds.feeds.is_empty() {
            if self.active_feed_index - 1 >= 0 {
                self.active_feed_index -= 1;
                self.draw(cache);
            }
        }
    }

    fn get_subscriptions_cols(&self, cache: &Cache) -> Vec<Vec<String>> {
        let mut list_cols: Vec<Vec<String>> = vec![];
        for subscription in &cache.subscriptions.subscriptions {
            list_cols.push(vec![
                subscription.get_total_feed_unread(&cache.db_connection).to_string(),
                String::from(subscription.title()) // TODO use ref ?
            ]);
        }
        list_cols
    }

    fn get_feeds_cols(&self, cache: &Cache) -> Vec<Vec<String>> {
        let mut list_feeds: Vec<Vec<String>> = vec![];
        for feed in &cache.feeds.feeds {
            list_feeds.push(vec![String::from(feed.title.as_ref())]);
        }
        list_feeds
    }
}

impl Component for MainDisplayControllers {

    fn on_init(&mut self, cache: &Cache) {
        self.draw(cache);
    }

    fn on_key_down(&mut self, cache: &Cache) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "subscriptions" => {
                self.set_next_active_sub_index(cache);
            },
            "feeds" => {
                self.set_next_active_feed_index(cache);
            },
            "read" => {
                // TODO scroll down
            },
            _ => {}
        }
    }

    fn on_key_up(&mut self, cache: &Cache) {
        match self.current_window.as_ref() {
            "subscriptions" => {
                self.set_previous_active_sub_index(cache);
            },
            "feeds" => {
                self.set_previous_active_feed_index(cache);
            },
            "read" => {
                // TODO scroll up
            },
            _ => {}
        }
    }

    fn on_key_enter(&mut self, cache: &mut Cache) {
        match self.current_window.as_ref() {
            "subscriptions" => {
                /* Clear feeds */
                cache.feeds.clear();
                /* Get active subscription id */
                // TODO improve here
                let id_sub = cache.subscriptions.subscriptions
                    .get(self.active_subscription_index as usize)
                    .unwrap()
                    .id;
                /* Fetch feeds from db */
                cache.feeds = get_feeds_from_subscription(
                    &cache.db_connection,
                    id_sub
                );
                /* Init active item index */
                self.active_feed_index = 0;
                /* Load the feeds screen */
                self.current_window = String::from("feeds");
                self.draw(cache);
            },
            "feeds" => {
                if !cache.feeds.feeds.is_empty() {
                    self.current_window = String::from("read");
                    self.draw(cache);
                }
            },
            "read" => {
                // nothing happen here
            },
            _ => {}
        }
    }

    fn on_key_previous(&mut self, cache: &Cache) {
        match self.current_window.as_ref() {
            "subscriptions" => {
                // nothing here
            },
            "feeds" => {
                self.current_window = String::from("subscriptions");
                self.draw(cache);
            },
            "read" => {
                self.current_window = String::from("feeds");
                self.draw(cache);
            },
            _ => {}
        }
    }

    fn on_synchronize_start(&mut self, _cache: &mut Cache) {}

    fn on_synchronize_done(&mut self, cache: &mut Cache) {
        self.draw(cache);
    }
    fn on_channel_synchronize_start(&mut self, cache: &mut Cache, channel_name: &str) {
        self.draw(cache);
    }
}