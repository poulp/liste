extern crate ncurses;

use app::Cache;
use models::items::Item;

const ROW_HEADER_HEIGHT: i32 = 1;

pub struct WindowListItems {
    /* Display info */
    active_item_index: i32,
    /* Only display items between this index
     * (using to scroll)
     */
    data_start_index: i32,
    data_end_index: i32,
    active_item_display_index: i32,

    /* Window */
    window: ncurses::WINDOW,
    height: i32,
    width: i32,
    startx: i32,
}

impl WindowListItems {
    pub fn new() -> WindowListItems {
        let height = ncurses::LINES() - 2;
        let width = ncurses::COLS();
        let startx = 0;
        let starty = 1;
        let window = ncurses::newwin(height, width, starty, startx);

        WindowListItems {
            active_item_index: 0,
            active_item_display_index: 0,
            data_start_index: 0,
            data_end_index: 0,
            window,
            height,
            width,
            startx,
        }
    }

    pub fn set_cols_data(&mut self, cache: &Cache) {
        self.data_start_index = 0;
        let len_channels = cache.items.len() as i32;

        if len_channels > self.height {
            self.data_end_index = self.height - ROW_HEADER_HEIGHT;
        } else {
            self.data_end_index = len_channels;
        }
    }

    pub fn init_active_item_index(&mut self) {
        self.active_item_index = 0;
        self.active_item_display_index = 0;
    }

    pub fn get_active_item_index(&self) -> i32 {
        self.active_item_index
    }

    pub fn draw(&self, cache: &Cache) {
        if !cache.items.is_empty() {
            /* Display columns headers */
            self.print_header_row();
            /* Display each row of the list */
            let mut display_index = 0;
            for index in self.data_start_index..self.data_end_index {
                let item = cache.items.get(index as usize);
                match item {
                    Some(s) => {
                        self.print_item_row(s, display_index);
                        display_index += 1;
                    },
                    None => {}
                }
            }
            ncurses::wrefresh(self.window);
            /* Display active item row */
            let active_item = cache.items.get(
                self.active_item_index as usize);
            match active_item {
                Some(s) => {
                    self.print_active_item_row(s);
                },
                None => {}
            }
        } else {
            /* Nothing to display */
            ncurses::mvwprintw(self.window, 0, 0, "No items here ...");
            ncurses::wrefresh(self.window);
        }
    }

    pub fn draw_next_item(&mut self, cache: &Cache) {
        if !cache.items.is_empty() {
            if self.active_item_index + 1 < self.data_end_index {
                self.active_item_index += 1;
                self.active_item_display_index += 1;
                self.draw(cache);
            } else {
                if self.active_item_index + 1 < cache.items.len() as i32 {
                    self.active_item_index += 1;
                    self.data_start_index += 1;
                    self.data_end_index += 1;
                    self.draw(cache)
                }
            }
        }
    }

    pub fn draw_previous_item(&mut self, cache: &Cache) {
        if !cache.items.is_empty() {
            if self.active_item_index > self.data_start_index {
                self.active_item_index -= 1;
                self.active_item_display_index -= 1;
                self.draw(cache);
            } else {
                if self.active_item_index > 0 {
                    self.active_item_index -= 1;
                    self.data_start_index -= 1;
                    self.data_end_index -= 1;
                    self.draw(cache)
                }
            }
        }
    }

    fn print_header_row(& self) {
        /* exemple header : Date | Unread | Title */
        let final_display = String::from("Unread |Title");
        ncurses::wattr_on(self.window, ncurses::A_BOLD());
        ncurses::mvwprintw(
            self.window,
            0,
            0,
            final_display.as_ref());
        ncurses::wattr_off(self.window, ncurses::A_BOLD());
        ncurses::wrefresh(self.window);
    }

    fn format_item_row(&self, item: &Item) -> String {
        /* Display a row according to the headers */
        let mut final_display = String::from("");

        // Title
        final_display.push_str(item.title.as_ref());

        let cols = ncurses::COLS() as usize;

        /* Cut row if too long */
        if final_display.len() > cols {
            final_display.truncate(cols-4);
            final_display.push_str("...");
        }
        final_display
    }

    fn print_item_row(&self, item: &Item, index: i32) {
        let starty = index + 1;
        let display = self.format_item_row(item);

        if !item.is_read {
            ncurses::wattr_on(self.window, ncurses::A_BOLD());
            ncurses::mvwprintw(self.window, starty, 0, display.as_ref());
            ncurses::wattr_off(self.window, ncurses::A_BOLD());
        } else {
            ncurses::mvwprintw(self.window, starty, 0, display.as_ref());
        }
    }


    fn print_active_item_row(&self, item: &Item) {
        let row_height = 1;
        let starty = self.active_item_display_index + 2;
        let window_active_item = ncurses::newwin(
            row_height,
            self.width,
            starty,
            self.startx);
        let display_row = self.format_item_row(item);

        ncurses::wbkgd(window_active_item, ncurses::COLOR_PAIR(1));
        if !item.is_read {
            ncurses::wattr_on(window_active_item, ncurses::A_BOLD());
            ncurses::mvwprintw(window_active_item, 0, 0, display_row.as_ref());
            ncurses::wattr_off(window_active_item, ncurses::A_BOLD());
        } else {
            ncurses::mvwprintw(window_active_item, 0, 0, display_row.as_ref());
        }
        ncurses::wrefresh(window_active_item);
    }

    pub fn clear(& self) {
        ncurses::wclear(self.window);
    }
}
