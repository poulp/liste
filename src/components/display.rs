extern crate ncurses;

use super::super::database::{
    get_items_from_channel
};
use windows::list::WindowList;
use windows::text::WindowText;
use app::Cache;
use components::component::Component;

pub struct MainDisplayComponent {
    window_channels: WindowList,
    window_items: WindowList,
    window_item: WindowText,

    /* possible values :
     *  - channels
     *  - items
     *  - item
     */
    // TODO find a better way
    // TODO state pattern
    current_window: String,
}

impl MainDisplayComponent {
    pub fn new() -> MainDisplayComponent {
        let cols_channel = vec![
            (String::from("Unread"), 12),
            (String::from("Channel"), 16),
        ];
        let cols_items = vec![
            (String::from("Title"), 12)
        ];
        MainDisplayComponent {
            window_channels: WindowList::new(cols_channel),
            window_items: WindowList::new(cols_items),
            window_item: WindowText::new(),
            current_window: String::from("channels"),
        }
    }

    fn draw(&mut self, _cache: &Cache) {
        self.clear_windows();
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.draw();
            },
            "items" => {
                self.window_items.draw();
            },
            "item" => {
                self.window_item.draw();
            },
            _ => {}
        }
    }

    fn clear_windows(&mut self) {
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
            },
            "items" => {
                self.window_items.clear();
            },
            "item" => {
                self.window_item.clear();
            },
            _ => {}
        }
    }

    fn get_channels_cols(&self, cache: &Cache) -> Vec<Vec<String>> {
        let mut list_cols: Vec<Vec<String>> = vec![];
        for channel in &cache.channels.channels {
            list_cols.push(vec![
                channel.get_total_item_unread(&cache.db_connection).to_string(),
                String::from(channel.title()) // TODO use ref ?
            ]);
        }
        list_cols
    }

    fn get_items_cols(&self, cache: &Cache) -> Vec<Vec<String>> {
        let mut list_items: Vec<Vec<String>> = vec![];
        for item in &cache.items.items {
            list_items.push(vec![item.title.clone()]);
        }
        list_items
    }
}

impl Component for MainDisplayComponent {

    fn on_init(&mut self, cache: &Cache) {
        let channels_cols = self.get_channels_cols(cache);
        self.window_channels.init_active_item_index();
        self.window_channels.set_cols_data(channels_cols);
        self.draw(cache);
    }

    fn on_key_down(&mut self, _cache: &Cache) {
        // TODO move to window ?
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
                self.window_channels.draw_next_item();
            },
            "items" => {
                self.window_items.clear();
                self.window_items.draw_next_item();
            },
            "item" => {
                self.window_item.scroll_down();
            },
            _ => {}
        }
    }

    fn on_key_up(&mut self, _cache: &Cache) {
        match self.current_window.as_ref() {
            "channels" => {
                self.window_channels.clear();
                self.window_channels.draw_previous_item();
            },
            "items" => {
                self.window_items.clear();
                self.window_items.draw_previous_item();
            },
            "item" => {
                self.window_item.scroll_up();
            },
            _ => {}
        }
    }

    fn on_key_enter(&mut self, cache: &mut Cache) {
        match self.current_window.as_ref() {
            "channels" => {
                /* Clear items */
                cache.items.clear();
                /* Get active channel id */
                // TODO improve here
                let channel_id = cache.channels.channels
                    .get(self.window_channels.get_active_item_index() as usize)
                    .unwrap()
                    .id;
                /* Fetch items from db */
                cache.items = get_items_from_channel(
                    &cache.db_connection,
                    channel_id
                );
                let items_data = self.get_items_cols(cache);
                self.window_items.init_active_item_index();
                self.window_items.set_cols_data(items_data);
                /* Load the items screen */
                self.current_window = String::from("items");
                self.draw(cache);
            },
            "items" => {
                if !cache.items.items.is_empty() {
                    let item = cache.items.items.get(
                        self.window_items.get_active_item_index() as usize).unwrap();
                    self.window_item.set_item(item);
                    self.current_window = String::from("item");
                    self.draw(cache);
                }
            },
            "item" => {
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
            "items" => {
                self.current_window = String::from("channels");
                self.draw(cache);
            },
            "item" => {
                self.current_window = String::from("items");
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

    fn on_channel_synchronize_start(&mut self, _cache: &mut Cache, _channel_name: &str) {}

    fn on_channel_synchronize_done(&mut self, cache: &mut Cache) {
        let channels_cols = self.get_channels_cols(cache);
        self.window_channels.set_cols_data(channels_cols);
        self.draw(cache);
    }
}