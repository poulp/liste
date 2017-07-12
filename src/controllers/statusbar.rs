use windows::statusbar::WindowStatusBar;
use controllers::component::Component;
use app::Cache;

pub struct ControllerStatusBar {
    window: WindowStatusBar
}

impl ControllerStatusBar {
    pub fn new() -> ControllerStatusBar {
        let window = WindowStatusBar::new();
        ControllerStatusBar {
            window: window,
        }
    }

    pub fn draw_text(&mut self, text: String) {
        self.window.draw(text);
    }

    pub fn clear(&mut self) {
        self.window.clear();
    }

    pub fn draw_commands(&mut self) {
        self.window.draw(String::from("q: Quit, s: Synchronization"));
    }
}

impl Component for ControllerStatusBar {

    fn on_init(&mut self, _cache: &Cache) {
        self.draw_commands();
    }

    fn on_key_down(&mut self, _cache: &Cache) {}

    fn on_key_up(&mut self, _cache: &Cache) {}

    fn on_key_enter(&mut self, _cache: &mut Cache) {}

    fn on_key_previous(&mut self, _cache: &Cache) {}

    fn on_synchronize_start(&mut self, _cache: &mut Cache) {}

    fn on_synchronize_done(&mut self, _cache: &mut Cache) {
        self.draw_commands();
    }
    fn on_channel_synchronize_start(&mut self, cache: &mut Cache, channel_name: &str) {
        self.draw_text(String::from(channel_name));
    }
}