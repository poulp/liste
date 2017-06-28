extern crate clap;
extern crate ncurses;
extern crate liste;
extern crate rusqlite;

use std::process;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{mpsc, Mutex, Arc};

use clap::App;
use clap::Arg;
use rusqlite::Connection;

use liste::settings::Settings;
use liste::screen::Screen;
use liste::database::init_database;

const VERSION: &str = "0.0.1";
const MS_PER_FRAME: u64 = 40;

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

    /* Open database */
    let db_connection = Connection::open("base.db").unwrap();
    /* Create tables */
    init_database(&db_connection, &settings);

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
    {
        let (tx, rx) = mpsc::channel();
        let mut screen = Screen::new(
            &settings,
            &db_connection,
            tx.clone()
        );

        ncurses::refresh();
        screen.on_init();
        /* Event loop */
        loop {
            /* Get user input (async) */
            let ch = ncurses::getch();
            if screen.get_input(ch, &settings) {
                break;
            }
            /* Get event */
            match rx.try_recv() {
                Ok(event) => {
                    println!("{}", event);
                },
                Err(error) => {}
            }
            thread::sleep(Duration::from_millis(MS_PER_FRAME));
        }
    }
    // Stop ncurses
    ncurses::endwin();
    // Close database connection
    db_connection.close().unwrap();
}
