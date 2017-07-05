extern crate rusqlite;
extern crate rss;

use std::thread;
use std::sync::mpsc::Sender;

use self::rusqlite::Connection;
use self::rss::Channel;

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

    pub fn synchronize(&self, tx: Sender<String>) {
        thread::spawn(move || {
            let db_conn = Connection::open("base.db").unwrap();
            let subscriptions = get_subscriptions(&db_conn);
            let len_sub = subscriptions.subscriptions.len();

            for (index, subscription) in subscriptions.subscriptions.iter().enumerate() {
                tx.send(
                    format!("Download channels : {}/{}", index, len_sub)
                ).unwrap();
                // Download feeds
                let channel_opt = Channel::from_url(subscription.url.as_ref());
                match channel_opt {
                    Ok(channel) => {
                        db_conn.execute(
                            "UPDATE subscription SET title = ? WHERE subscription_id = ?",
                            &[&channel.title(), &subscription.id]
                        );
                        /* Fetch feeds */
                        for item in channel.items() {
                            /* Save feed in db */
                            create_feed(
                                &db_conn,
                                item.title().unwrap(),
                                item.description().unwrap(),
                                subscription.id
                            )
                        }

                    },
                    Err(error) => {
                        //self.status_bar.draw_text(String::from("error !"));
                    }
                }
                tx.send(
                    format!("Download channels : {len_sub}/{len_sub} Done !", len_sub=len_sub)
                ).unwrap();
            }
        });
    }
}