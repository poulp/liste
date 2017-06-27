extern crate ncurses;

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

    pub fn draw(&mut self, text: String) {
        ncurses::wrefresh(self.window);
        ncurses::box_(self.window, 0, 0);
        ncurses::mvwprintw(self.window,0 ,0 ,&text);
        ncurses::wrefresh(self.window);
    }
}