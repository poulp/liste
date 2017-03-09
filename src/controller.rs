extern crate ncurses;
extern crate feed;

use window::Window;
use model::{ListModel, Subscription};

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

pub struct Controller {
    window: Window,
    model: ListModel
}

impl Controller {
    pub fn new() -> Controller {

        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();

        let mut feed_window = Window::new("feed".to_string(), total_width, total_height);
        let mut list_model = ListModel::new();

        /* Read urls file */
        // Create a path to the desired file
        let path = Path::new("feeds");
        let display = path.display();

        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open"),
            Ok(file) => file
        };
        
        let buffer = BufReader::new(file);

        /* Add subscriptions */
        for line in buffer.lines() {
            let url = line.unwrap();
        //    let feed = feed::FeedBuilder::read_from_url(url).finalize();
          //  let channel = feed.channel();
            list_model.add_feed(url.to_string());
        }

        Controller {
            window: feed_window,
            model: list_model
        }
    }

    /*************************
     * CALLBACK
     ************************/

    pub fn on_init(&mut self) {
        self.window.draw(&self.model);
    }

    pub fn on_next_active_sub(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub + 1 < self.model.subscriptions.len() as i32 {
                self.window.active_sub += 1;
                self.window.draw(&self.model);
            }
        }
    }

    pub fn on_previous_active_sub(&mut self){
        if !self.model.subscriptions.is_empty() {
            if self.window.active_sub - 1 >= 0 {
                self.window.active_sub -= 1;
                self.window.draw(&self.model);
            }
        }
    }
}

