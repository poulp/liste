use app::Cache;

pub trait Component {
    fn on_init(&mut self, cache: &Cache);
    fn on_key_down(&mut self, cache: &Cache);
    fn on_key_up(&mut self, cache: &Cache);
    fn on_key_enter(&mut self, cache: &mut Cache);
    fn on_key_previous(&mut self, cache: &Cache);
    fn on_synchronize_start(&mut self, cache: &mut Cache);
    fn on_synchronize_done(&mut self, cache: &mut Cache);
    fn on_channel_synchronize_start(&mut self, cache: &mut Cache, channel_name: &str);
}