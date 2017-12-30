extern crate ncurses;
extern crate html5ever;

use std::default::Default;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{NodeData, RcDom, Handle};
use self::html5ever::tendril::{StrTendril, TendrilSink};

use super::super::models::items::Item;


pub fn get_lines_height_from_content(content: &str) -> i32 {
    let len = content.len() as f32;
    let cols = ncurses::COLS() as f32;
    let height = len / cols;
    height.ceil() as i32
}


pub struct WindowText {
    window: ncurses::WINDOW,
    title: String,
    content: String,
    height: i32,
    scroll: i32
}

impl WindowText {
    pub fn new() -> WindowText {
        let total_width = ncurses::COLS();
        let total_height = ncurses::LINES() - 1;
        let startx = 0;
        let starty = 0;
        let window = ncurses::newwin(
            total_height,
            total_width,
            starty,
            startx
        );
        ncurses::scrollok(window, true);

        WindowText {
            window: window,
            content: String::from(""),
            title: String::from(""),
            height: 0,
            scroll: 0
        }
    }

    pub fn set_item(&mut self, item: &Item) {
        self.content = item.description.clone();
        self.title = item.title.clone();
        self.scroll = 0;
    }

    fn draw_title(&mut self) -> i32 {
        // Draw the title of the item and return
        // the height of the display in term of lines
        ncurses::wattr_on(self.window, ncurses::A_BOLD() | ncurses::A_UNDERLINE());
        ncurses::mvwprintw(self.window, 0 ,0, self.title.as_ref());
        ncurses::wattr_off(self.window, ncurses::A_BOLD() | ncurses::A_UNDERLINE());
        /* Height taken by the title */
        get_lines_height_from_content(self.title.as_ref())
    }

    fn draw_node(&mut self, index: i32, handle: Handle) -> i32 {
        let mut content_height = index;

        match handle.data {
            NodeData::Document => {},
            NodeData::Text { ref contents } => {
                ncurses::mvwprintw(self.window, index, 0, &contents.borrow());
                content_height += get_lines_height_from_content(&contents.borrow());
            },
            NodeData::Doctype { .. } => {},
            NodeData::Comment { .. } => { },
            NodeData::Element { ref name, .. } => {
                match name.local.as_ref() {
                    "br" => {
                        content_height +=1;
                    },
                    "b" => {
                        ncurses::wattr_on(self.window, ncurses::A_BOLD());
                    },
                    _ => {}
                }
            },
            NodeData::ProcessingInstruction { .. }=> {}
        }

        for child in handle.children.borrow().iter() {
            content_height = self.draw_node(content_height, child.clone());
        }

        match handle.data {
            NodeData::Document => {},
            NodeData::Text { .. } => {},
            NodeData::Doctype { .. } => {},
            NodeData::Comment { .. } => { },
            NodeData::Element { ref name, .. } => {
                match name.local.as_ref() {
                    "b" => {
                        ncurses::wattr_off(self.window, ncurses::A_BOLD());
                    },
                    _ => {}
                }
            },
            NodeData::ProcessingInstruction { .. }=> {}
        }

        return content_height;
    }

    pub fn draw(&mut self) {
        let str_tendril = StrTendril::from(self.content.as_ref());
        let dom = parse_document(RcDom::default(), Default::default()).one(str_tendril);
        let height_title = self.draw_title();
        self.height = self.draw_node(height_title+1, dom.document);
        ncurses::wrefresh(self.window);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll < self.height - 1 {
            ncurses::wclear(self.window);
            self.draw();
            self.scroll += 1;
            ncurses::wscrl(self.window, self.scroll);
            ncurses::wrefresh(self.window);
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            ncurses::wclear(self.window);
            self.draw();
            self.scroll -= 1;
            ncurses::wscrl(self.window, self.scroll);
            ncurses::wrefresh(self.window);
        }
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.window);
        ncurses::wrefresh(self.window);
    }
}