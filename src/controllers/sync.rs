extern crate rusqlite;
extern crate rss;

use std::thread;

use self::rusqlite::Connection;
use self::rss::Channel;

use controllers::component::Component;
use app::Cache;

use database::{
    get_channels,
    create_item
};

pub struct ControllerSync {
}

impl ControllerSync {
    pub fn new() -> ControllerSync {
        ControllerSync{}
    }
}

impl Component for ControllerSync {

    fn on_init(&mut self, _cache: &Cache) {}

    fn on_key_down(&mut self, _cache: &Cache) {}

    fn on_key_up(&mut self, _cache: &Cache) {}

    fn on_key_enter(&mut self, _cache: &mut Cache) {}

    fn on_key_previous(&mut self, _cache: &Cache) {}

    fn on_synchronize_start(&mut self, cache: &mut Cache) {
        /// Fetch channels and save items
        let tx = cache.tx.clone();
        // TODO one thread per channel
        thread::spawn(move || {
            let db_conn = Connection::open("base.db").unwrap();
            let channels = get_channels(&db_conn);

            for channel in channels.channels.iter() {
                tx.send(
                    format!("{}", channel.title())
                ).unwrap();
                // Download feeds
                let channel_opt = Channel::from_url(channel.link.as_ref());
                match channel_opt {
                    Ok(channel_fetched) => {
                        db_conn.execute(
                            "UPDATE channel SET title = ?, description = ? WHERE channel_id = ?",
                            &[&channel_fetched.title(), &channel_fetched.description(), &channel.id]
                        ).unwrap();

                        /* Fetch feeds */
                        for item in channel_fetched.items() {
                            /* Save feed in db */
                            create_item(
                                &db_conn,
                                item.link().unwrap(),
                                item.title().unwrap(),
                                item.description().unwrap(),
                                channel.id);
                        }

                    },
                    Err(_error) => {}
                };
                tx.send(
                    format!("cdone")
                ).unwrap();
            }
            tx.send(
                format!("done")
            ).unwrap();
            db_conn.close().unwrap();
        });
    }

    fn on_synchronize_done(&mut self, _cache: &mut Cache) {}

    fn on_channel_synchronize_start(&mut self, _cache: &mut Cache, _channel_name: &str) {}

    fn on_channel_synchronize_done(&mut self, _cache: &mut Cache) {}
}