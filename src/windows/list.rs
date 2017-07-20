extern crate ncurses;

const ROW_HEADER_HEIGHT: i32 = 1;

pub struct WindowList {
    /* Columns headers */
    cols_header: Vec<(String, i32)>,
    cols_data: Vec<Vec<String>>,

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
    starty: i32
}

impl WindowList {
    pub fn new(cols_header: Vec<(String, i32)>) -> WindowList {
        let height = ncurses::LINES() - 1;
        let width = ncurses::COLS();
        let startx = 0;
        let starty = 0;
        let window = ncurses::newwin(height, width, starty, startx);

        WindowList {
            cols_header: cols_header,
            cols_data: vec![],
            active_item_index: 0,
            active_item_display_index: 0,
            data_start_index: 0,
            data_end_index: 0,
            window: window,
            height: height,
            width: width,
            startx: startx,
            starty: starty
        }
    }

    pub fn set_cols_data(&mut self, data: Vec<Vec<String>>) {
        self.cols_data = data;
        self.data_start_index = 0;

        if self.cols_data.len() as i32 > self.height {
            self.data_end_index = self.height - ROW_HEADER_HEIGHT;
        } else {
            self.data_end_index = self.cols_data.len() as i32;
        }
    }

    pub fn init_active_item_index(&mut self) {
        self.active_item_index = 0;
        self.active_item_display_index = 0;
    }

    pub fn get_active_item_index(&self) -> i32 {
        self.active_item_index
    }

    pub fn draw(&self) {
        /* Display columns headers */
        self.print_header_row();
        if !self.cols_data.is_empty() {
            /* Display each row of the list */
            let mut display_index = 0;
            for index in self.data_start_index..self.data_end_index {
                let cols = self.cols_data.get(index as usize).unwrap();
                self.print_item_row(cols, display_index);
                display_index += 1;
            }
            ncurses::wrefresh(self.window);
            /* Display active item row */
            let active_item_display = self.format_item_row(
                self.cols_data.get(
                    self.active_item_index as usize).unwrap());
            self.print_active_item_row(active_item_display);
        }
    }

    pub fn draw_next_item(&mut self) {
        if !self.cols_data.is_empty() {
            if self.active_item_index + 1 < self.data_end_index {
                self.active_item_index += 1;
                self.active_item_display_index += 1;
                self.draw();
            } else {
                if self.active_item_index + 1 < self.cols_data.len() as i32 {
                    self.active_item_index += 1;
                    self.data_start_index += 1;
                    self.data_end_index += 1;
                    self.draw()
                }
            }
        }
    }

    pub fn draw_previous_item(&mut self) {
        if !self.cols_data.is_empty() {
            if self.active_item_index > self.data_start_index {
                self.active_item_index -= 1;
                self.active_item_display_index -= 1;
                self.draw();
            } else {
                if self.active_item_index > 0 {
                    self.active_item_index -= 1;
                    self.data_start_index -= 1;
                    self.data_end_index -= 1;
                    self.draw()
                }
            }
        }
    }

    fn print_header_row(& self) {
        /* exemple header : Date | Unread | Title */
        if !self.cols_header.is_empty() {
            let mut final_display = String::from("");

            for (index, cols_tuple) in self.cols_header.iter().enumerate() {
                let col_name = &cols_tuple.0;
                let col_width = &cols_tuple.1;
                let is_last_cols = self.cols_header.len() -1 == index;

                final_display.push_str(col_name.as_ref());
                if !is_last_cols && col_name.len() <= *col_width as usize {
                    for _ in col_name.len()..*col_width as usize {
                        final_display.push_str(" ");
                    }
                }
                /* Separator */
                if !is_last_cols {
                    final_display.push_str("|");
                }
            }
            ncurses::wattr_on(self.window, ncurses::A_BOLD());
            ncurses::mvwprintw(
                self.window,
                self.starty,
                self.startx,
                final_display.as_ref());
            ncurses::wattr_off(self.window, ncurses::A_BOLD());
            ncurses::wrefresh(self.window);
        }
    }

    fn format_item_row(&self, cols: &Vec<String>) -> String {
        if !self.cols_header.is_empty() {
            let mut final_display = String::from("");
            for (index, cols_tuple) in self.cols_header.iter().enumerate() {
                let is_last_cols = self.cols_header.len() -1 == index;
                let col_width = cols_tuple.1;

                final_display.push_str(cols[index].as_ref());
                if !is_last_cols && cols[index].len() <= col_width as usize {
                    for _ in cols[index].len()..col_width as usize {
                        final_display.push_str(" ");
                    }
                }
                if !is_last_cols {
                    final_display.push_str("|");
                }
            }
            final_display
        } else {
            cols[0].clone() // TODO ref ?
        }
    }

    fn print_item_row(&self, cols: &Vec<String>, index: i32) {
        let starty = index + 1;
        let display = self.format_item_row(cols);
        ncurses::mvwprintw(self.window, starty, self.startx, display.as_ref());
    }


    fn print_active_item_row(&self, display: String) {
        let row_height = 1;
        let starty = self.active_item_display_index + 1;
        let window_active_item = ncurses::newwin(
            row_height,
            self.width,
            starty,
            self.startx);

        ncurses::wbkgd(window_active_item, ncurses::COLOR_PAIR(1));
        ncurses::mvwprintw(window_active_item, 0, 0, display.as_ref());
        ncurses::wrefresh(window_active_item);
    }

    pub fn clear(& self) {
        ncurses::wclear(self.window);
    }
}