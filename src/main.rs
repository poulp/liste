extern crate ncurses;

mod controller;

use std::fmt::Debug;
use std::process;
use std::time::{Duration, Instant};
use std::thread;
/*
fn create_window(width: i32, height: i32, starty: i32, startx: i32) -> ncurses::WINDOW {
    let window = ncurses::newwin(width, height, starty, startx);
    ncurses::box_(window, 0, 0);
    ncurses::wrefresh(window);
    window
}

fn destroy_window(window: ncurses::WINDOW) {
    ncurses::wborder(window, 1, 1, 1, 1, 1, 1, 1, 1);
    ncurses::wrefresh(window);
    ncurses::delwin(window);
}

fn main() {

    *//****************
     * Initialisation
     ****************//*

    let list_subscription = vec![
        "Salut",
        "Bonjour",
        "Hello"
    ];

    *//* Start ncruses *//*
    ncurses::initscr();

        if(!ncurses::has_colors())
	{	ncurses::endwin();
		println!("Your terminal does not support color");
		process::exit(1);
	}

    *//* Configuration *//*
    ncurses::noecho(); // Don't echo while getch
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Hide cursor
    ncurses::start_color(); // Enable colors
    ncurses::init_pair(1, ncurses::COLOR_RED, ncurses::COLOR_BLUE);

    let total_width = ncurses::COLS();
    let total_height = ncurses::LINES();

    let mut startx = total_width/2;
    let mut starty = total_height/2;

    ncurses::printw("Press F1 to exit");
    ncurses::refresh();

    let mut feed_window = create_window(10, 20, starty, startx);


    let mut n = 1;
    for sub in &list_subscription {
        if n == 1 {
            ncurses::wattr_on(feed_window, ncurses::A_UNDERLINE() | ncurses::COLOR_PAIR(1));
            ncurses::mvwprintw(feed_window, n, 1, sub);
            ncurses::wattr_off(feed_window, ncurses::A_UNDERLINE() | ncurses::COLOR_PAIR(1));
        } else {
            ncurses::mvwprintw(feed_window, n, 1, sub);
        }
        n += 1;
    }
    ncurses::wclear(feed_window);
    ncurses::wrefresh(feed_window);


//    loop {
//        let ch = ncurses::getch();
//        match ch {
//            113 => break,
//            ncurses::KEY_LEFT => {
//                destroy_window(new_window);
//                startx -= 1;
//                new_window = create_window(10, 10, starty, startx);
//            },
//            _ => {
//                ncurses::printw("test");
//                ncurses::refresh();
//            }
//        }
//    }

    let ch = ncurses::getch();

    *//* Stop ncurses *//*
    ncurses::endwin();
}*/





fn main() {
    // Start ncurses
    ncurses::initscr();

    if !ncurses::has_colors() {
        ncurses::endwin();
        println!("Your terminal does not support color");
        process::exit(1);
    }

    // Configuration
    ncurses::noecho(); // Don't echo while getch
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Hide cursor
    ncurses::start_color(); // Enable colors
    ncurses::init_pair(1, ncurses::COLOR_RED, ncurses::COLOR_BLUE);
    ncurses::timeout(0); // non blocking io


    const MS_PER_FRAME: u64 = 60;

    let total_width = ncurses::COLS();
    let total_height = ncurses::LINES();

    let mut feed_window = Window::new("feed".to_string(), total_width, total_height);


    let sub_1 = Subscription::new("monflux".to_string());
    let sub_2 = Subscription::new("monflux number 2".to_string());
    let mut list_model = ListModel::new();
    list_model.add_feed(&sub_1);
    list_model.add_feed(&sub_2);

    let mut controller = Controller::new(&mut feed_window, &list_model);

    controller.on_init();

    /* Event loop */
    loop {
        //let start = Instant::now();
        /* getch is async */
        let ch = ncurses::getch();
        match ch {
            ncurses::KEY_DOWN => {
                /* Go to the next sub */
                controller.on_next_active_sub();
            },
            ncurses::KEY_UP => {
                /* Go to the previous sub */
                controller.on_previous_active_sub();
            },
            113 => break, // 'q' -> quit
            _ => {} // do nothing
        }
        //let end = Instant::now();
        //let sleep_time = start.elapsed().as_secs() + MS_PER_FRAME - end.elapsed().as_secs();
        thread::sleep(Duration::from_millis(MS_PER_FRAME));
    }

    //Stop ncurses
    ncurses::endwin();
}
