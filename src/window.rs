extern crate ncurses;

use model::{ListModel, Subscription};

pub struct WindowSubscriptions {
    pub name: String,
    pub active_sub: i32,
    feed_windows: Vec<ncurses::WINDOW>
}

impl WindowSubscriptions {
    pub fn new(name: String, width: i32, height: i32) -> WindowSubscriptions {
        WindowSubscriptions {
            name: name,
            active_sub: 0,
            feed_windows: vec![]
        }
    }

    fn create_widget(width: i32, height: i32, starty: i32, startx: i32) -> ncurses::WINDOW {
        let window = ncurses::newwin(width, height, starty, startx);
        ncurses::box_(window, 0, 0);
        ncurses::wrefresh(window);
        window
    }

    fn create_window_row_feed(&self, feed: &Subscription, index: i32) -> ncurses::WINDOW {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES();
        let startx = 1;
        let starty = index;
        let height = 1;

        let window = ncurses::newwin(height, total_width, starty, startx);
        if self.active_sub == index {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::wprintw(window, &feed.name);
        } else {
            ncurses::wprintw(window, &feed.name);
        }
        ncurses::wrefresh(window);
        window
    }

    pub fn draw(&mut self, model: &ListModel) {
        /* Clean the windows feed list */
        self.feed_windows.clear();
        /* Clear the screen */
        ncurses::refresh();
        for (index, feed) in model.subscriptions.iter().enumerate() {
            let window = self.create_window_row_feed(&feed, index as i32);
            self.feed_windows.push(window);
        }
    }
}

pub struct WindowStatusBar {
    window: ncurses::WINDOW
}

impl WindowStatusBar {
    pub fn new() -> WindowStatusBar {
        let total_width = ncurses::COLS() - 1;
        let total_height = 2;
        let startx = 1;
        let starty = ncurses::LINES() - 2;
        ncurses::refresh();

        let window = ncurses::newwin(total_height, total_width, starty, startx);
        ncurses::box_(window, 0, 0);
        ncurses::wprintw(window, "status");
        ncurses::wrefresh(window);
        ncurses::refresh();

        WindowStatusBar {
            window: window
        }
    }

    pub fn draw(&mut self){
        ncurses::refresh();
    }
}