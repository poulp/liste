extern crate ncurses;

use super::super::database::{
    get_feeds_from_channel
};
use windows::list::WindowList;
use windows::text::WindowText;
use app::Cache;
use controllers::component::Component;

pub struct MainDisplayControllers {
    window_channels: WindowList,
    window_feeds: WindowList,
    window_feed: WindowText,

    /* possible values :
     *  - channels
     *  - feeds
     *  - read
     */
    // TODO find a better way
    // TODO state pattern
    current_window: String,
}

impl MainDisplayControllers {
    pub fn new() -> MainDisplayControllers {
        let cols_channel = vec![
            (String::from("Unread"), 12),
            (String::from("Channel"), 16),
        ];
        let cols_feeds = vec![
            (String::from("Title"), 12)
        ];
        MainDisplayControllers {
            window_channels: WindowList::new(cols_channel),
            window_feeds: WindowList::new(cols_feeds),
            window_feed: WindowText::new(),
            current_window: String::from("channels"),
        }
    }

    fn draw(&mut self, cache: &Cache) {
        self.clear_windows();
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.draw();
            },
            "feeds" => {
                self.window_feeds.draw();
            },
            "read" => {
                self.window_feed.draw();
            },
            _ => {}
        }
    }

    fn clear_windows(&mut self) {
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
            },
            "feeds" => {
                self.window_feeds.clear();
            },
            "read" => {
                self.window_feed.clear();
            },
            _ => {}
        }
    }

    fn get_channels_cols(&self, cache: &Cache) -> Vec<Vec<String>> {
        let mut list_cols: Vec<Vec<String>> = vec![];
        for channel in &cache.channels.channels {
            list_cols.push(vec![
                channel.get_total_feed_unread(&cache.db_connection).to_string(),
                String::from(channel.title()) // TODO use ref ?
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
        let mut channels_cols = self.get_channels_cols(cache);
        self.window_channels.set_cols_data(channels_cols);
        self.draw(cache);
    }

    fn on_key_down(&mut self, cache: &Cache) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
                self.window_channels.draw_next_item();
            },
            "feeds" => {
                self.window_feeds.clear();
                self.window_feeds.draw_next_item();
            },
            "read" => {
                self.window_feed.scroll_down();
            },
            _ => {}
        }
    }

    fn on_key_up(&mut self, cache: &Cache) {
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
                self.window_channels.draw_previous_item();
            },
            "feeds" => {
                self.window_feeds.clear();
                self.window_feeds.draw_previous_item();
            },
            "read" => {
                self.window_feed.scroll_up();
            },
            _ => {}
        }
    }

    fn on_key_enter(&mut self, cache: &mut Cache) {
        match self.current_window.as_ref() {
            "channels" => {
                /* Clear feeds */
                cache.feeds.clear();
                /* Get active channel id */
                // TODO improve here
                let channel_id = cache.channels.channels
                    .get(self.window_channels.get_active_item_index() as usize)
                    .unwrap()
                    .id;
                /* Fetch feeds from db */
                cache.feeds = get_feeds_from_channel(
                    &cache.db_connection,
                    channel_id
                );
                let feeds_data = self.get_feeds_cols(cache);
                self.window_feeds.set_cols_data(feeds_data);
                /* Load the feeds screen */
                self.current_window = String::from("feeds");
                self.draw(cache);
            },
            "feeds" => {
                if !cache.feeds.feeds.is_empty() {
                    let mut feed = cache.feeds.feeds.get(
                        self.window_feeds.get_active_item_index() as usize).unwrap();
                    self.window_feed.set_feed(feed);
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
            "channels" => {
                // nothing here
            },
            "feeds" => {
                self.current_window = String::from("channels");
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
        let channels_cols = self.get_channels_cols(cache);
        self.window_channels.set_cols_data(channels_cols);
        self.draw(cache);
    }

    fn on_channel_synchronize_start(&mut self, cache: &mut Cache, channel_name: &str) {}

    fn on_channel_synchronize_done(&mut self, cache: &mut Cache) {
        let channels_cols = self.get_channels_cols(cache);
        self.window_channels.set_cols_data(channels_cols);
        self.draw(cache);
    }
}