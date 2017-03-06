extern crate ncurses;

extern crate liste;
use liste::controller::Controller;
use liste::window::Window;
use liste::model::Subscription;
use liste::model::ListModel;

use std::fmt::Debug;
use std::process;
use std::time::{Duration, Instant};
use std::thread;

static COLOR_BACKGROUND: i16 = 16;

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
    ncurses::timeout(0); // non blocking io

    /* Colors */
    ncurses::start_color(); // Enable colors
    //ncurses::init_color(COLOR_BACKGROUND, 0, 43 * 4, 54 * 4);
    ncurses::init_pair(1, ncurses::COLOR_BLACK, ncurses::COLOR_WHITE);


    const MS_PER_FRAME: u64 = 60;



//    let mut feed_window = Window::new("feed".to_string(), total_width, total_height);
//
//
//    let sub_1 = Subscription::new("monflux".to_string());
//    let sub_2 = Subscription::new("monflux number 2".to_string());
//    let mut list_model = ListModel::new();
//    list_model.add_feed(&sub_1);
//    list_model.add_feed(&sub_2);
//
//    let mut controller = Controller::new(&mut feed_window, &list_model);
    let mut controller = Controller::new();

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
