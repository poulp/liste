extern crate clap;
extern crate ncurses;
extern crate liste;

use std::process;

use clap::App;
use clap::Arg;

use liste::settings::Settings;
use liste::app::Application;

const VERSION: &str = "0.0.1";

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
        println!("Error settings: {}", err);
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

    ncurses::start_color(); // Enable colors
    ncurses::init_pair(1, ncurses::COLOR_BLACK, ncurses::COLOR_WHITE);
    ncurses::init_pair(2, ncurses::COLOR_WHITE, ncurses::COLOR_GREEN);
    {
        let mut app = Application::new(&settings);
        app.main_loop()
    }
    // Stop ncurses
    ncurses::endwin();
}
