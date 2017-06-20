pub mod statusbar;
pub mod display;

pub trait Controller {
    fn on_init(&mut self);
    fn on_key_down(&mut self);
    fn on_key_up(&mut self);
    fn on_key_enter(&mut self);
    fn on_key_previous(&mut self);
}