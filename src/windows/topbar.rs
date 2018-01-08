extern crate ncurses;

pub struct WindowTopBar {
    window: ncurses::WINDOW
}

impl WindowTopBar {
    pub fn new() -> WindowTopBar {
        let total_width = ncurses::COLS();
        let total_height = 1;
        let startx = 0;
        let starty = 0;
        let window = ncurses::newwin(total_height, total_width, starty, startx);

        WindowTopBar {
            window: window
        }
    }

    pub fn draw(&mut self, text: String) {
        let display = format!("{} - {}", self.get_header(), text);

        self.clear();
        ncurses::wbkgd(self.window, ncurses::COLOR_PAIR(1));
        ncurses::wrefresh(self.window);
        ncurses::wattr_on(self.window, ncurses::A_BOLD());
        ncurses::mvwprintw(self.window, 0 , 0, display.as_ref());
        ncurses::wattr_off(self.window, ncurses::A_BOLD());
        ncurses::wrefresh(self.window);
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.window);
        ncurses::wrefresh(self.window);
    }

    fn get_header(&self) -> &str {
        "List v0.1"
    }
}