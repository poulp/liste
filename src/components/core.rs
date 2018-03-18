extern crate ncurses;

use super::super::database::{
    get_items_from_channel
};
use windows::list_channels::WindowListChannels;
use windows::list_items::WindowListItems;
use windows::text::WindowText;
use windows::topbar::WindowTopBar;
use app::Cache;
use components::component::Component;

enum WindowState {
    Channels,
    Items,
    Item
}

pub struct ComponentCore {
    window_channels: WindowListChannels,
    window_items: WindowListItems,
    window_item: WindowText,
    window_topbar: WindowTopBar,
    current_window: WindowState,
}

impl ComponentCore {
    pub fn new() -> ComponentCore {
        ComponentCore {
            window_channels: WindowListChannels::new(),
            window_items: WindowListItems::new(),
            window_item: WindowText::new(),
            window_topbar: WindowTopBar::new(),
            current_window: WindowState::Channels,
        }
    }

    fn draw(&mut self, cache: &Cache) {
        self.clear_windows();
        match self.current_window {
            WindowState::Channels => {
                self.window_topbar.draw(String::from("Channels"));
                self.window_channels.draw(cache);
            },
            WindowState::Items => {
                /* Get active channel */
                let active_channel = cache.channels
                    .get(self.window_channels.get_active_item_index() as usize);
                /* Display active channel title on top bar */
                match active_channel {
                    Some(channel) => { self.window_topbar.draw(String::from(channel.title())); },
                    None => { self.window_topbar.draw(String::from("")); }
                }
                /* Draw the list of items */
                self.window_items.draw(cache);
            },
            WindowState::Item => {
                self.window_item.draw();
            },
        }
    }

    fn clear_windows(&mut self) {
        match self.current_window {
            WindowState::Channels => {
                self.window_channels.clear();
            },
            WindowState::Items => {
                self.window_items.clear();
            },
            WindowState::Item => {
                self.window_item.clear();
            },
        }
    }
}

impl Component for ComponentCore {

    fn on_init(&mut self, cache: &Cache) {
        self.window_channels.init_active_item_index();
        self.window_channels.set_cols_data(cache);
        self.draw(cache);
    }

    fn on_key_down(&mut self, cache: &Cache) {
        match self.current_window {
            WindowState::Channels => {
                self.window_channels.clear();
                self.window_channels.draw_next_item(cache);
            },
            WindowState::Items => {
                self.window_items.clear();
                self.window_items.draw_next_item(cache);
            },
            WindowState::Item => {
                self.window_item.scroll_down();
            },
        }
    }

    fn on_key_up(&mut self, cache: &Cache) {
        match self.current_window {
            WindowState::Channels => {
                self.window_channels.clear();
                self.window_channels.draw_previous_item(cache);
            },
            WindowState::Items => {
                self.window_items.clear();
                self.window_items.draw_previous_item(cache);
            },
            WindowState::Item => {
                self.window_item.scroll_up();
            },
        }
    }

    fn on_key_enter(&mut self, cache: &mut Cache) {
        match self.current_window {
            WindowState::Channels => {
                /* Clear items */
                cache.items.clear();
                /* Get active channel id */
                let channel = cache.channels
                    .get(self.window_channels.get_active_item_index() as usize);

                match channel {
                    Some(channel) => {
                        /* Fetch items from db.
                         * Be careful if some thread is writing db */
                        match cache.db_lock.try_lock() {
                            Ok(_) => {
                                cache.items = get_items_from_channel(
                                    &cache.db_connection,
                                    channel.id
                                );
                            },
                            Err(_) => {}
                        }
                        self.window_items.init_active_item_index();
                        self.window_items.set_cols_data(cache);
                        /* Load the items screen */
                        self.current_window = WindowState::Items;
                        self.draw(cache);
                    },
                    None => {}
                }
            },
            WindowState::Items => {
                if !cache.items.is_empty() {
                    cache.set_item_as_read(self.window_items.get_active_item_index());
                    let item = cache.items.get(
                        self.window_items.get_active_item_index() as usize).unwrap();
                    self.window_item.set_item(item);
                    self.current_window = WindowState::Item;
                    self.draw(cache);
                }
            },
            WindowState::Item => {
                // nothing happen here
            },
        }
    }

    fn on_key_previous(&mut self, cache: &mut Cache) {
        match self.current_window {
            WindowState::Channels => {
                // nothing here
            },
            WindowState::Items => {
                self.current_window = WindowState::Channels;
                cache.refresh();
                self.draw(cache);
            },
            WindowState::Item => {
                self.current_window = WindowState::Items;
                self.draw(cache);
            },
        }
    }

    fn on_synchronize_start(&mut self, _cache: &mut Cache) {}

    fn on_synchronize_done(&mut self, cache: &mut Cache) {
        match cache.db_lock.try_lock() {
            Ok(_) => {
                self.window_channels.set_cols_data(cache);
                self.draw(cache);
            },
            Err(_) => {}
        }
    }
}
