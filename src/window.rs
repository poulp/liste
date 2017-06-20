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
        ncurses::refresh();
        //ncurses::box_(self.window, 0, 0);
        ncurses::wprintw(self.window, "status");
        ncurses::wrefresh(self.window);
        //ncurses::refresh();
    }
}

pub struct WindowListView {
    windows: Vec<ncurses::WINDOW>,
    pub active_item: i32
}

impl WindowListView {
    pub fn new() -> WindowListView {
        WindowListView {
            active_item: 0,
            windows: vec![]
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
        if self.active_item == index {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::wprintw(window, item.get_name());
        } else {
            ncurses::wprintw(window, item.get_name());
        }
        ncurses::wrefresh(window);
        ncurses::refresh();
        window
    }

    pub fn draw<I: TraitListViewItem>(&mut self, list: &Vec<I>) {
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

pub struct WindowTextView {
    window: ncurses::WINDOW,
}

impl WindowTextView {
    pub fn new(text: String) -> WindowTextView {
        let total_width = ncurses::COLS() - 1;
        let total_height = ncurses::LINES() - 2;
        let startx = 1;
        let starty = 1;

        WindowTextView {
            window: ncurses::newwin(
                total_height,
                total_width,
                starty,
                startx
            )
        }
    }

    pub fn draw(&mut self, text: String) {
        ncurses::wprintw(self.window, text.as_ref());
    }
}