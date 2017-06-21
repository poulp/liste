extern crate clap;
extern crate ncurses;
extern crate liste;

use liste::settings::Settings;
use liste::screen::Screen;

use std::process;
use std::time::{Duration, Instant};
use std::thread;

use clap::App;
use clap::Arg;

const VERSION: &str = "0.0.1";
//const COLOR_BACKGROUND: i16 = 16;
const MS_PER_FRAME: u64 = 60;

fn main() {

    let matches = App::new("Liste")
        .version(VERSION)
        .arg(Arg::with_name("settings")
            .short("s")
            .long("settings")
            .value_name("FILE")
            .help("Sets a custom settings file")
            .takes_value(true))
        .get_matches();

    /* Get settings */

    let settings = Settings::new(matches).unwrap_or_else(|err| {
        println!("Problem with settings: {}", err);
        process::exit(1);
    });

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

    let mut screen = Screen::new(&settings);
    ncurses::refresh();
    screen.on_init();
    /* Event loop */
    loop {
        //let start = Instant::now();
        /* getch is async */
        let ch = ncurses::getch();
        if screen.get_input(ch, &settings) {
            break;
        }
        //let end = Instant::now();
        //let sleep_time = start.elapsed().as_secs() + MS_PER_FRAME - end.elapsed().as_secs();
        thread::sleep(Duration::from_millis(MS_PER_FRAME));
    }

    //Stop ncurses
    ncurses::endwin();
}
