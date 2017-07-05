extern crate ncurses;



pub struct WindowList {
    /* Col name and width */
    cols: Vec<(String, i32)>
}

impl WindowList {
    pub fn new(cols: Vec<(String, i32)>) -> WindowList {
        WindowList {
            cols: cols
        }
    }

    fn create_title_row(& self) {
        if !self.cols.is_empty() {
            let mut final_display = String::from("");
            let total_width = ncurses::COLS() - 1;
            let window = ncurses::newwin(1, total_width, 0, 1);

            for (index, cols_tuple) in self.cols.iter().enumerate() {
                let col_name = &cols_tuple.0;
                let col_width = &cols_tuple.1;
                let is_last_cols = self.cols.len() -1 == index;

                final_display.push_str(col_name.as_ref());
                if !is_last_cols && col_name.len() <= *col_width as usize {
                    for _ in col_name.len()..*col_width as usize {
                        final_display.push_str(" ");
                    }
                }
                if !is_last_cols {
                    final_display.push_str("|");
                }
            }
            ncurses::wattr_on(window, ncurses::A_BOLD());
            ncurses::mvwprintw(window, 0, 0, final_display.as_ref());
            ncurses::wattr_off(window, ncurses::A_BOLD());
            ncurses::wrefresh(window);
        }
    }


    fn get_row_display(&self, cols: &Vec<String>) -> String {
        if !self.cols.is_empty() {
            let mut final_display = String::from("");
            for (index, cols_tuple) in self.cols.iter().enumerate() {
                let is_last_cols = self.cols.len() -1 == index;
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

    fn create_window_row(&self, cols: &Vec<String>,
                         index: i32, active_item: bool) {
        let total_width = ncurses::COLS() - 1;
        let startx = 1;
        let starty = index + 1;
        let row_height = 1;
        let row_width = total_width;

        let window = ncurses::newwin(row_height, row_width, starty, startx);
        let display = self.get_row_display(cols);
        if active_item {
            ncurses::wbkgd(window, ncurses::COLOR_PAIR(1));
        }
        ncurses::mvwprintw(window, 0, 0, display.as_ref());
        ncurses::wrefresh(window);
    }

    pub fn draw(& self, active_item_index: i32, list_cols: &Vec<Vec<String>>) {
        self.create_title_row();
        for (index, cols) in list_cols.iter().enumerate() {
            self.create_window_row(
                cols,
                index as i32,
                active_item_index == index as i32
            );
        }
    }

    pub fn clear(& self) {
        let window = ncurses::newwin(ncurses::LINES() - 3, ncurses::COLS(), 0, 0);
        ncurses::wclear(window);
        ncurses::wrefresh(window);
    }
}