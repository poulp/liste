use model;

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
//
//    fn next_active_sub(&mut self, subscriptions: &Vec<&Subscription>){
//        if !subscriptions.is_empty() {
//            if self.active_sub + 1 < subscriptions.len() as i32 {
//                self.active_sub += 1;
//            }
//        }
//    }
//
//    fn previous_active_sub(&mut self, model: &ListModel){
//        if !model.subscriptions.is_empty() {
//            if self.active_sub - 1 >= 0 {
//                self.active_sub -= 1;
//            }
//        }
//    }

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