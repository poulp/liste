use windows::statusbar::WindowStatusBar;
use components::component::Component;
use app::Cache;

pub struct ComponentStatusBar {
    window: WindowStatusBar
}

impl ComponentStatusBar {
    pub fn new() -> ComponentStatusBar {
        let window = WindowStatusBar::new();
        ComponentStatusBar {
            window: window,
        }
    }

    pub fn draw_text(&mut self, text: String) {
        self.window.draw(text);
    }

    pub fn draw_commands(&mut self) {
        self.window.draw(String::from("q: Quit, s: Synchronization, r: Back"));
    }
}

impl Component for ComponentStatusBar {

    fn on_init(&mut self, _cache: &Cache) {
        self.draw_commands();
    }

    fn on_key_down(&mut self, _cache: &Cache) {}

    fn on_key_up(&mut self, _cache: &Cache) {}

    fn on_key_enter(&mut self, _cache: &mut Cache) {}

    fn on_key_previous(&mut self, _cache: &Cache) {}

    fn on_synchronize_start(&mut self, _cache: &mut Cache) {
        self.draw_text(format!("Synchronization in progress ..."));
    }

    fn on_synchronize_done(&mut self, _cache: &mut Cache) {
        self.draw_commands();
    }
}