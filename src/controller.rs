extern crate ncurses;
extern crate feed;

use window::WindowSubscriptions;
use window::WindowStatusBar;
use window::WindowListView;
use model::{ListModel, Subscription};
use settings::Settings;

use std::process;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

pub trait Controller {
    //fn new(settings: &Settings) -> Self;
    fn on_init(&mut self);
    fn on_key_down(&mut self);
    fn on_key_up(&mut self);
}

pub struct ControllerSubscriptions {
    window: WindowSubscriptions,
    model: ListModel
}

impl ControllerSubscriptions {
    pub fn new(settings: &Settings) -> ControllerSubscriptions {

        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 4;

        let mut feed_window = WindowSubscriptions::new("feed".to_string(), total_width, total_height);
        let mut list_model = ListModel::new();

        /* Urls file */
        let path = Path::new(&settings.settings_file);
        let display = path.display();

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
            list_model.add_feed(url.to_string());
        }

        ControllerSubscriptions {
            window: feed_window,
            model: list_model
        }
    }
}

impl Controller for ControllerSubscriptions {

    /*************************
     * CALLBACK
     ************************/

    fn on_init(&mut self) {
        self.window.draw(&self.model);
    }

    fn on_key_down(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub + 1 < self.model.subscriptions.len() as i32 {
                self.window.active_sub += 1;
                self.window.draw(&self.model);
            }
        }
    }

    fn on_key_up(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub - 1 >= 0 {
                self.window.active_sub -= 1;
                self.window.draw(&self.model);
            }
        }
    }
}

pub struct ControllerStatusBar {
    window: WindowStatusBar
}

impl ControllerStatusBar {
    pub fn new(settings: &Settings) -> ControllerStatusBar {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();

        let mut window = WindowStatusBar::new();

        ControllerStatusBar {
            window: window,
        }
    }
}

impl Controller for ControllerStatusBar {


    /*************************
     * CALLBACK
     ************************/

    fn on_init(&mut self) {
        self.window.draw();
    }

    fn on_key_down(&mut self){}

    fn on_key_up(&mut self){}

}

pub struct ControllerFeeds {
    window: WindowListView,
    url: String,
    feeds: Vec<String>
}

impl ControllerFeeds {
    pub fn new(settings: &Settings, url: String) -> ControllerFeeds {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();

        let mut window = WindowListView::new();

        ControllerFeeds {
            window: window,
            feeds: vec![
                String::from("salut"),
                String::from("test")
            ],
            url: url
        }
    }
}

impl Controller for ControllerFeeds {


    /*************************
     * CALLBACK
     ************************/

    fn on_init(&mut self) {
        self.window.draw(&self.feeds);
    }

    fn on_key_down(&mut self){}

    fn on_key_up(&mut self){}

}