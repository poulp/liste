extern crate ncurses;

use super::super::models::listview::TraitListViewItem;

pub struct WindowList {}

impl WindowList {
    pub fn new() -> WindowList {
        WindowList {}
    }

    fn create_window_row<I: TraitListViewItem>(&self, item: &I,
                                               index: i32, active_item: bool) {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 3;
        let startx = 1;
        let starty = index;
        let row_height = 1;
        let row_width = total_width;

        let window = ncurses::newwin(row_height, row_width, starty, startx);
        if active_item {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
            ncurses::mvwprintw(window, 0, 0, item.get_name().as_ref());
        } else {
            ncurses::mvwprintw(window, 0, 0, item.get_name().as_ref());
        }
        ncurses::wrefresh(window);
    }

    pub fn draw<I: TraitListViewItem>(&mut self, active_item_index: i32, list: &Vec<I>) {
        for (index, feed) in list.iter().enumerate() {
            self.create_window_row(
                feed,
                index as i32,
                active_item_index == index as i32
            );
        }
    }

    pub fn clear(&mut self) {
        let window = ncurses::newwin(ncurses::LINES() - 3, ncurses::COLS(), 0, 0);
        ncurses::wclear(window);
        ncurses::wrefresh(window);
    }
}