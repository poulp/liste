extern crate ncurses;

fn say_hello(name: i32) {
    let display_text: String = format!("Welcome {} \n", name);
    ncurses::printw(&display_text);
}

fn main() {

    /****************
     * Initialisation
     ****************/

    /* Start ncruses */
    ncurses::initscr();
    ncurses::noecho(); // Don't echo while getch
    ncurses::keypad(ncurses::stdscr(), true);

    /*** GO ***/

    ncurses::printw("Hello, what is your name ?\n");
    let ch: i32 = ncurses::getch();

    if ch == ncurses::KEY_F1 {
        say_hello(ch);
        ncurses::printw("KEY F1 !!");
    } else {
        say_hello(ch);
        ncurses::printw("Another key pressed: \n");
        ncurses::attron(ncurses::A_BOLD());
        ncurses::printw(&ch.to_string());
        ncurses::attroff(ncurses::A_BOLD());
    }

    ncurses::refresh();
    ncurses::getch();

    /* Stop ncurses */
    ncurses::endwin();
}