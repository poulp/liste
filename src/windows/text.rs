extern crate ncurses;

use super::super::models::feeds::Feed;

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

    pub fn draw(&mut self, feed: &Feed) {
        ncurses::mvwprintw(self.window, 0, 0, &feed.description);
        ncurses::wrefresh(self.window);
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.window);
        ncurses::wrefresh(self.window);
    }
}