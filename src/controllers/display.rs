extern crate ncurses;

use super::Controller;
use super::super::window::{
    WindowList,
    WindowText
};
use super::super::models::subscriptions::{
    Subscription,
    ListSubscriptions
};
use super::super::models::feeds::{
    Feed,
    ListFeeds
};
use super::super::settings::Settings;

use std::process;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


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

        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 4;

        //let mut feed_window = WindowSubscriptions::new("feed".to_string(), total_width, total_height);
        let mut list_model = ListSubscriptions::new();
        let mut feeds = ListFeeds::new();
        feeds.add_feed(String::from("test"));
        feeds.add_feed(String::from("salut"));

        /* Urls file */
        let path = Path::new(&settings.settings_file);
        //let display = path.display();

        /* Open urls file */
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => {
                // Quit ncurses
                ncurses::endwin();
                match path.to_str() {
                    Some(s) => {
                        println!("There is a problem with the urls file at {} :\n {}", s, why);
                    },
                    None => {
                        println!("There is a problem with the urls file :\n {}", why);
                    }
                }
                process::exit(1)
            },
        };

        let buffer = BufReader::new(file);

        /* Extract feeds urls */
        for line in buffer.lines() {
            let url = line.unwrap();
            /* Add subscription to the model */
            list_model.add_subscription(url.to_string());
        }

        MainDisplayControllers {
            window_subscriptions: WindowList::new(),
            window_feeds: WindowList::new(),
            window_feed: WindowText::new(),
            subscriptions: list_model,
            feeds: feeds,
            feed: String::from("--"),
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