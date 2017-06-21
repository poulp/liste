extern crate ncurses;

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