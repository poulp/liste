extern crate rusqlite;
extern crate rss;

use std::thread;

use self::rusqlite::Connection;
use self::rss::Channel;

use controllers::component::Component;
use app::Cache;

use database::{
    get_subscriptions,
    create_feed
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
        let tx = cache.tx.clone();
        // TODO one thread per channel
        thread::spawn(move || {
            let db_conn = Connection::open("base.db").unwrap();
            let subscriptions = get_subscriptions(&db_conn);

            for subscription in subscriptions.subscriptions.iter() {
                tx.send(
                    format!("{}", subscription.title())
                ).unwrap();
                // Download feeds
                let channel_opt = Channel::from_url(subscription.url.as_ref());
                match channel_opt {
                    Ok(channel) => {
                        db_conn.execute(
                            "UPDATE subscription SET title = ? WHERE subscription_id = ?",
                            &[&channel.title(), &subscription.id]
                        ).unwrap();

                        /* Fetch feeds */
                        for item in channel.items() {
                            /* Save feed in db */
                            create_feed(
                                &db_conn,
                                item.title().unwrap(),
                                item.description().unwrap(),
                                subscription.id);
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

    fn on_channel_synchronize_start(&mut self, cache: &mut Cache, channel_name: &str) {}

    fn on_channel_synchronize_done(&mut self, cache: &mut Cache) {}
}