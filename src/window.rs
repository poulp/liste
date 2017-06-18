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
        let total_height = ncurses::LINES() - 2;
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

//pub struct WindowFeeds {
//    window: ncurses::WINDOW,
//    pub active_sub: i32,
//    feed_windows: Vec<ncurses::WINDOW>
//}
//
//impl WindowFeeds {
//    pub fn new() -> WindowFeeds {
//        let total_width = ncurses::COLS() - 1;
//        let total_height = ncurses::LINES() - 2;
//        let startx = 1;
//        let starty = 0;
//        ncurses::refresh();
//
//        let window = ncurses::newwin(total_height, total_width, starty, startx);
//        ncurses::box_(window, 0, 0);
//        ncurses::wprintw(window, "feeds");
//        ncurses::wrefresh(window);
//        ncurses::refresh();
//
//        WindowFeeds {
//            window: window,
//            active_sub: 0,
//            feed_windows: vec![]
//        }
//    }
//
//    fn create_window_row_feed(&self, feed: String, index: i32) -> ncurses::WINDOW {
//        let total_width = ncurses::COLS();
//        let total_height = ncurses::LINES() - 2;
//        let startx = 1;
//        let starty = index;
//        let height = 1;
//
//        let window = ncurses::newwin(height, total_width, starty, startx);
//        if self.active_sub == index {
//            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
//            ncurses::wprintw(window, &feed);
//        } else {
//            ncurses::wprintw(window, &feed);
//        }
//        ncurses::wrefresh(window);
//        window
//    }
//
//    pub fn draw(&mut self, feeds: Vec<String>){
//        ncurses::refresh();
//         for (index, &feed) in feeds.iter().enumerate() {
//            let window = self.create_window_row_feed(feed, index as i32);
//            self.feed_windows.push(window);
//        }
//    }
//}

pub struct WindowListView {
    windows: Vec<ncurses::WINDOW>,
    active_item: i32
}

impl WindowListView {
    pub fn new() -> WindowListView {
        WindowListView {
            active_item: 0,
            windows: vec![]
        }
    }

    fn create_window_row(&self, feed: &String, index: i32) -> ncurses::WINDOW {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 2;
        let startx = 1;
        let starty = index;
        let row_height = 1;
        let row_width = total_width;

        let window = ncurses::newwin(row_height, row_width, starty, startx);
        if self.active_item == index {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::wprintw(window, &feed);
        } else {
            ncurses::wprintw(window, &feed);
        }
        ncurses::wrefresh(window);
        ncurses::refresh();
        window
    }

    pub fn draw(&mut self, list: &Vec<String>) {
        /* Clean the windows feed list */
        self.windows.clear();
        /* Clear the screen */
        ncurses::refresh();
        for (index, feed) in list.iter().enumerate() {
            let window = self.create_window_row(feed, index as i32);
            self.windows.push(window);
        }
    }
}