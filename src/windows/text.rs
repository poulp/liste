extern crate ncurses;

use super::super::models::items::Item;

pub struct WindowText {
    window: ncurses::WINDOW,
    content: String,
    height: i32,
    scroll: i32
}

impl WindowText {
    pub fn new() -> WindowText {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 1;
        let startx = 0;
        let starty = 0;
        let window = ncurses::newwin(
            total_height,
            total_width,
            starty,
            startx
        );
        ncurses::scrollok(window, true);

        WindowText {
            window: window,
            content: String::from(""),
            height: 0,
            scroll: 0
        }
    }

    pub fn set_item(&mut self, item: &Item) {
        self.content = item.description.clone();
    }

    pub fn draw(&mut self) {
        self.height = 0;
        let sp = self.content.split("\n");
        for (index, line) in sp.enumerate() {
            ncurses::mvwprintw(self.window, index as i32, 0, line);
            self.height += 1;
        }
        ncurses::wrefresh(self.window);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll < self.height {
            self.draw();
            self.scroll += 1;
            ncurses::wscrl(self.window, self.scroll);
            ncurses::wrefresh(self.window);
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.draw();
            self.scroll -= 1;
            ncurses::wscrl(self.window, self.scroll);
            ncurses::wrefresh(self.window);
        }
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.window);
        ncurses::wrefresh(self.window);
    }
}