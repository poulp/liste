extern crate reqwest;
extern crate rss;
extern crate rusqlite;
extern crate atom_syndication;

use std::thread;
use std::io::Read;
use std::io::Error;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::process;

use self::rusqlite::Connection;
use self::rss::Channel as RSSChannel;
use self::atom_syndication::Feed;

use components::component::Component;
use app::Cache;
use super::super::models::channels::Channel;

use database::{
    get_channels,
    create_item
};

fn content_from_url(link: &str) -> Result<String, FetchError> {
    /// Download content
    let mut content = String::new();
    reqwest::get(link)?.read_to_string(&mut content)?;
    Ok(content)
}

fn update_channel(db_connection: &Connection, title: &str, description: &str, channel_id: i32) -> Result<i32, rusqlite::Error>{
    db_connection.execute(
        "UPDATE channel SET title = ?, description = ? WHERE channel_id = ?",
        &[&title, &description, &channel_id]
    )
}

pub enum FetchError {
    UrlRequest(reqwest::Error),
    Io(Error)
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::UrlRequest(err)
    }
}

impl From<Error> for FetchError {
    fn from(err: Error) -> FetchError {
        FetchError::Io(err)
    }
}

pub struct ComponentSync {}

impl ComponentSync {
    pub fn new() -> ComponentSync {
        ComponentSync{}
    }

    fn sychronize_channels(&mut self, cache: &mut Cache, channels: Vec<Channel>) {
        let tx = cache.tx.clone();
        let db_lock = cache.db_lock.clone();
        thread::spawn(move || {

            let db_conn = Connection::open("base.db").unwrap();
            //let channels = get_channels(&db_conn);
            //let mut handles = vec![];

            for channel in channels {
                let channel_content = content_from_url(channel.link.as_ref());

                match channel_content {
                    Ok(content) => {
                        // Try RSS
                        let channel_rss_result = RSSChannel::from_str(content.as_ref());
                        match channel_rss_result {
                            Ok(channel_rss) => {
                                /* note : if you write :
                                 * let _ = lock_clone.lock().unwrap();
                                 * the lock is non blocking */
                                match db_lock.lock() {
                                    Ok(lock) => {
                                        let res_update_chan = update_channel(
                                            &db_conn,
                                            &channel_rss.title(),
                                            &channel_rss.description(),
                                            channel.id
                                        );
                                    },
                                    Err(_) => {}
                                }

                                //
                                //                                match res_update_chan {
                                //                                    Ok(res_update_chan) => {}
                                //                                    Err(err) => {
                                //                                        println!("Error update channel: {}", err);
                                //                                        process::exit(1);
                                //                                    }
                                //                                }

                                /* Fetch feeds */
                                for item in channel_rss.items() {
                                    /* Save feed in db */
                                    if let (Some(link), Some(title), Some(description)) =
                                    (item.link(), item.title(), item.description()) {

                                        match db_lock.lock() {
                                            Ok(lock) => {
                                                create_item(
                                                    &db_conn,
                                                    link,
                                                    title,
                                                    description,
                                                    channel.id);
                                            },
                                            Err(_) => {}
                                        }
                                        //                                        create_item(
                                        //                                            &db_conn,
                                        //                                            link,
                                        //                                            title,
                                        //                                            description,
                                        //                                            channel.id);

                                    }
                                }
                            },
                            Err(_) => {
                                // If RSS fail try Atom
                                //                                let channel_atom_result = Feed::from_str(content.as_ref());
                                //                                match channel_atom_result {
                                //                                    Ok(channel_atom) => {
                                //                                        let _data = db_lock.lock().unwrap();
                                //                                        update_channel(
                                //                                            &db_conn,
                                //                                            &channel_atom.title(),
                                //                                            &channel_atom.subtitle().unwrap_or(""),
                                //                                            channel.id
                                //                                        );
                                //                                        /* Fetch entries */
                                //                                        for entry in channel_atom.entries() {
                                //                                            if let Some(content) = entry.content() {
                                //                                                let link_opt = entry.links().into_iter().find(|&x| x.rel() == "self");
                                //                                                if let (Some(link), title, Some(value)) =
                                //                                                (link_opt, entry.title(), content.value()) {
                                //                                                    create_item(
                                //                                                        &db_conn,
                                //                                                        link.href(),
                                //                                                        title,
                                //                                                        value,
                                //                                                        channel.id);
                                //                                                }
                                //                                            }
                                //                                        }
                                //                                    },
                                //                                    Err(_) => {}
                                //                                }
                            }
                        }
                    },
                    Err(_) => {}
                }
                tx.send(
                    format!("cdone")
                ).unwrap();
            }
            db_conn.close().unwrap();
            //            tx.send(
            //                format!("done")
            //            ).unwrap();
        });
    }
}

impl Component for ComponentSync {

    fn on_init(&mut self, _cache: &Cache) {}

    fn on_key_down(&mut self, _cache: &Cache) {}

    fn on_key_up(&mut self, _cache: &Cache) {}

    fn on_key_enter(&mut self, _cache: &mut Cache) {}

    fn on_key_previous(&mut self, _cache: &Cache) {}

    fn on_synchronize_start(&mut self, cache: &mut Cache) {
        /// Fetch channels and save items
        /// RSS and ATOM support
        // TODO one thread per channel
        //self.sychronize_thread(cache, 0, 2);
        let db_conn = Connection::open("base.db").unwrap();
        let channels = get_channels(&db_conn);
        self.sychronize_channels(cache, channels.channels.get(0..2).unwrap().to_vec());
        self.sychronize_channels(cache, channels.channels.get(2..4).unwrap().to_vec());

        //        let tx = cache.tx.clone();
        //        let db_lock = cache.db_lock.clone();
        //        thread::spawn(move || {
        //            let db_conn = Connection::open("base.db").unwrap();
        //            let channels = get_channels(&db_conn);
        //            let mut handles = vec![];
        //
        //            for channel in channels.channels.into_iter() {
        //                let tx_thread = tx.clone();
        //                let db_lock = db_lock.clone();
        //
        //                let handle = thread::spawn(move || {
        //                    let tx_thread_thread = tx_thread.clone();
        //                    let db_thread = Connection::open("base.db").unwrap();
        //                    // Download items
        //                    let channel_content = content_from_url(channel.link.as_ref());
        //
        //                    match channel_content {
        //                        Ok(content) => {
        //                            // Try RSS
        //                            let channel_rss_result = RSSChannel::from_str(content.as_ref());
        //                            match channel_rss_result {
        //                                Ok(channel_rss) => {
        //                                    /* note : if you write :
        //                                     * let _ = lock_clone.lock().unwrap();
        //                                     * the lock is non blocking */
        //                                    let _data = db_lock.lock().unwrap();
        //                                    update_channel(
        //                                        &db_thread,
        //                                        &channel_rss.title(),
        //                                        &channel_rss.description(),
        //                                        channel.id
        //                                    );
        //
        //                                    /* Fetch feeds */
        //                                    for item in channel_rss.items() {
        //                                        /* Save feed in db */
        //                                        if let (Some(link), Some(title), Some(description)) =
        //                                        (item.link(), item.title(), item.description()) {
        //                                            create_item(
        //                                                &db_thread,
        //                                                link,
        //                                                title,
        //                                                description,
        //                                                channel.id);
        //                                        }
        //                                    }
        //                                },
        //                                Err(_) => {
        //                                    // If RSS fail try Atom
        //                                    let channel_atom_result = Feed::from_str(content.as_ref());
        //                                    match channel_atom_result {
        //                                        Ok(channel_atom) => {
        //                                            let _data = db_lock.lock().unwrap();
        //                                            update_channel(
        //                                                &db_thread,
        //                                                &channel_atom.title(),
        //                                                &channel_atom.subtitle().unwrap_or(""),
        //                                                channel.id
        //                                            );
        //                                            /* Fetch entries */
        //                                            for entry in channel_atom.entries() {
        //                                                if let Some(content) = entry.content() {
        //                                                    let link_opt = entry.links().into_iter().find(|&x| x.rel() == "self");
        //                                                    if let (Some(link), title, Some(value)) =
        //                                                    (link_opt, entry.title(), content.value()) {
        //                                                        create_item(
        //                                                            &db_thread,
        //                                                            link.href(),
        //                                                            title,
        //                                                            value,
        //                                                            channel.id);
        //                                                    }
        //                                                }
        //                                            }
        //                                        },
        //                                        Err(_) => {}
        //                                    }
        //                                }
        //                            }
        //                        },
        //                        Err(_) => {}
        //                    }
        //                    tx_thread_thread.send(
        //                        format!("cdone")
        //                    ).unwrap();
        //                    db_thread.close().unwrap();
        //                });
        //                handles.push(handle);
        //            }
        //            for handle in handles {
        //                handle.join().unwrap();
        //            }
        //            tx.send(
        //                format!("done")
        //            ).unwrap();
        //        });
    }

    fn on_synchronize_done(&mut self, _cache: &mut Cache) {}
}