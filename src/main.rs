extern crate ncurses;

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

struct Subscription {
    name: String,
}

impl Subscription {

    fn new(name: String) -> Subscription {
        Subscription {
            name: name
        }
    }
}

struct ListModel<'a> {
    active_sub: i32,
    subscriptions: Vec<&'a Subscription>,
    observers: Vec<&'a Window>
}

impl<'a> ListModel<'a> {

    fn new() -> ListModel<'a> {
        ListModel {
            observers: vec![],
            subscriptions:vec![],
            active_sub: 0
        }
    }

    fn add_feed(&mut self, feed: &'a Subscription) {
        self.subscriptions.push(feed);
        self.notify_observers();
    }

    fn next_active_feed(&mut self) {
        self.active_sub += 1;
    }

    fn previous_active_feed(&mut self) {
        self.active_sub += 1;
    }


    fn register_observer(&mut self, window: &'a Window) {
        self.observers.push(window);
    }

    fn notify_observers(&self) {
        for obs in &self.observers {
            obs.on_notify(self);
        }
    }
}

struct Window {
    name: String,
    widget: ncurses::WINDOW,
    active_sub: i32
}

impl Window {

    fn new(name: String, width: i32, height: i32) -> Window {
        let window = Window::create_widget(width/2, height/2, 10 , 10);
        Window {
            name: name,
            widget: window,
            active_sub: 0
        }
    }

    fn create_widget(width: i32, height: i32, starty: i32, startx: i32) -> ncurses::WINDOW {
        let window = ncurses::newwin(width, height, starty, startx);
        ncurses::box_(window, 0, 0);
        ncurses::wrefresh(window);
        window
    }

    fn next_active_sub(&mut self, model: &ListModel){
        if !model.subscriptions.is_empty() {
            if self.active_sub + 1 < model.subscriptions.len() as i32 {
                self.active_sub += 1;
            }
        }
    }

    fn previous_active_sub(&mut self, model: &ListModel){
        if !model.subscriptions.is_empty() {
            if self.active_sub - 1 >= 0 {
                self.active_sub -= 1;
            }
        }
    }

    fn draw(&self, model: &ListModel) {
        ncurses::clear();
        for (index, feed) in model.subscriptions.iter().enumerate() {
            if self.active_sub == index as i32 {
                ncurses::attron(ncurses::A_BOLD());
                ncurses::printw(&feed.name);
                ncurses::attroff(ncurses::A_BOLD());
            } else {
                ncurses::printw(&feed.name);
            }
        }
        ncurses::refresh();
    }

    fn on_notify(&self, model: &ListModel) {
        self.draw(model);
    }
}

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
    list_model.register_observer(&feed_window);
    list_model.add_feed(&sub_1);
    list_model.add_feed(&sub_2);
    //list_model.notify_observers();
    //list_model.add_feed("test feed".to_string());

    loop {
        //let start = Instant::now();
        let ch = ncurses::getch();
        match ch {
            ncurses::KEY_DOWN => {
               feed_window.next_active_sub(&list_model);
            },
            ncurses::KEY_UP => {
                feed_window.previous_active_sub(&list_model);
            },
            113 => break, // quit
            _ => {} // do nothing
        }
        //let end = Instant::now();
        //let sleep_time = start.elapsed().as_secs() + MS_PER_FRAME - end.elapsed().as_secs();
        thread::sleep(Duration::from_millis(MS_PER_FRAME));
    }

    //Stop ncurses
    ncurses::endwin();
}
