extern crate ncurses;

pub struct WindowStatusBar {
    window: ncurses::WINDOW
}

impl WindowStatusBar {
    pub fn new() -> WindowStatusBar {
        let total_width = ncurses::COLS();
        let total_height = 1;
        let startx = 0;
        let starty = ncurses::LINES() - 1;
        let window = ncurses::newwin(total_height, total_width, starty, startx);

        WindowStatusBar {
            window: window
        }
    }

    pub fn draw(&mut self, text: String) {
        self.clear();
        ncurses::wbkgd(self.window, ncurses::COLOR_PAIR(1));
        ncurses::wrefresh(self.window);
        ncurses::wattr_on(self.window, ncurses::A_BOLD());
        ncurses::mvwprintw(self.window, 0 , 1, &text);
        ncurses::wattr_off(self.window, ncurses::A_BOLD());
        ncurses::wrefresh(self.window);
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.window);
        ncurses::wrefresh(self.window);
    }
}