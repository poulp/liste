extern crate ncurses;

use window::Window;
use model::{ListModel, Subscription};

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
        list_model.add_feed("monflux".to_string());
        list_model.add_feed("monflux2".to_string());
        list_model.add_feed("monflux2".to_string());

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