extern crate ncurses;

use super::super::models::listview::TraitListViewItem;

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
        let total_height = ncurses::LINES() - 3;
        let startx = 1;
        let starty = index;
        let row_height = 1;
        let row_width = total_width;

        let window = ncurses::newwin(row_height, row_width, starty, startx);
        if self.active_item == index {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::mvwprintw(window, 0, 0, item.get_name());
        } else {
            ncurses::mvwprintw(window, 0, 0, item.get_name());
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
        let window = ncurses::newwin(ncurses::LINES() - 3, ncurses::COLS(), 0, 0);
        ncurses::wclear(window);
        ncurses::wrefresh(window);
    }
}