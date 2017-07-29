extern crate ncurses;

pub fn get_lines_height_from_content(content: &str) -> i32 {
    let len = content.len() as f32;
    let cols = ncurses::COLS() as f32;
    let height = len / cols;
    height.ceil() as i32
}