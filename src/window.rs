extern crate ncurses;

use models::subscriptions::{ListSubscriptions, Subscription};
use models::listview::TraitListViewItem;

pub struct WindowStatusBar {
    window: ncurses::WINDOW
}

impl WindowStatusBar {
    pub fn new() -> WindowStatusBar {
        let total_width = ncurses::COLS() - 1;
        let total_height = 2;
        let startx = 1;
        let starty = ncurses::LINES() - 2;
        let window = ncurses::newwin(total_height, total_width, starty, startx);

        WindowStatusBar {
            window: window
        }
    }

    pub fn draw(&mut self){
        //ncurses::wrefresh(self.window);
        ncurses::box_(self.window, 0, 0);
        ncurses::wprintw(self.window, "status");
        ncurses::wrefresh(self.window);
        //ncurses::refresh();
    }
}

pub struct WindowList {
    pub active_item: i32
}

impl WindowList {
    pub fn new() -> WindowList {
        WindowList {
            active_item: 0,
        }
    }

    fn create_window_row<I: TraitListViewItem>(&self, item: &I, index: i32) -> ncurses::WINDOW {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 2;
        let startx = 1;
        let starty = index;
        let row_height = 1;
        let row_width = total_width;

        let window = ncurses::newwin(row_height, row_width, starty, startx);
        ncurses::refresh();
        if self.active_item == index {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::wprintw(window, item.get_name());
        } else {
            ncurses::wprintw(window, item.get_name());
        }
        ncurses::wrefresh(window);
        window
    }

    pub fn draw<I: TraitListViewItem>(&mut self, list: &Vec<I>) {
        for (index, feed) in list.iter().enumerate() {
            let window = self.create_window_row(feed, index as i32);
        }
    }

    pub fn clear(&mut self) {
        let window = ncurses::newwin(ncurses::LINES() - 2, ncurses::COLS(), 1, 1);
        ncurses::wclear(window);
        ncurses::wrefresh(window);
    }
}

pub struct WindowText {
    window: ncurses::WINDOW,
}

impl WindowText {
    pub fn new() -> WindowText {
        let total_width = ncurses::COLS() - 1;
        let total_height = ncurses::LINES() - 2;
        let startx = 1;
        let starty = 0;

        WindowText {
            window: ncurses::newwin(
                total_height,
                total_width,
                starty,
                startx
            )
        }
    }

    pub fn draw(&mut self, text: &str) {
        ncurses::wprintw(self.window, text);
        ncurses::wrefresh(self.window);
    }
}