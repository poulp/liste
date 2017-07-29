extern crate reqwest;
extern crate rss;
extern crate rusqlite;
extern crate atom_syndication;

use std::thread;
use std::io::Read;
use std::io::Error;
use std::str::FromStr;

use self::rusqlite::Connection;
use self::rss::Channel;
use self::atom_syndication::Feed;

use components::component::Component;
use app::Cache;

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

fn update_channel(db_connection: &Connection, title: &str, description: &str, channel_id: i32) {
    db_connection.execute(
        "UPDATE channel SET title = ?, description = ? WHERE channel_id = ?",
        &[&title, &description, &channel_id]
    ).unwrap();
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
        let tx = cache.tx.clone();
        // TODO one thread per channel
        thread::spawn(move || {
            let db_conn = Connection::open("base.db").unwrap();
            let channels = get_channels(&db_conn);

            for channel in channels.channels.iter() {
                tx.send(
                    format!("{}", channel.title())
                ).unwrap();
                // Download items
                let channel_content = content_from_url(channel.link.as_ref());

                match channel_content {
                    Ok(content) => {
                        // Try RSS
                        let channel_rss_result = Channel::from_str(content.as_ref());
                        match channel_rss_result {
                            Ok(channel_rss) => {
                                update_channel(
                                    &db_conn,
                                    &channel_rss.title(),
                                    &channel_rss.description(),
                                    channel.id
                                );
                                /* Fetch feeds */
                                for item in channel_rss.items() {
                                    /* Save feed in db */
                                    if let (Some(link), Some(title), Some(description)) =
                                           (item.link(), item.title(), item.description()) {
                                        create_item(
                                            &db_conn,
                                            link,
                                            title,
                                            description,
                                            channel.id);
                                    }
                                }
                            },
                            Err(_) => {
                                // Try Atom
                                let channel_atom_result = Feed::from_str(content.as_ref());
                                match channel_atom_result {
                                    Ok(channel_atom) => {
                                        update_channel(
                                            &db_conn,
                                            &channel_atom.title(),
                                            &channel_atom.subtitle().unwrap_or(""),
                                            channel.id
                                        );
                                        /* Fetch entries */
                                        for entry in channel_atom.entries() {
                                            if let Some(content) = entry.content() {
                                                let link_opt = entry.links().into_iter().find(|&x| x.rel() == "self");
                                                if let (Some(link), title, Some(value)) =
                                                       (link_opt, entry.title(), content.value()) {
                                                    create_item(
                                                        &db_conn,
                                                        link.href(),
                                                        title,
                                                        value,
                                                        channel.id);
                                                }
                                            }
                                        }
                                    },
                                    Err(_) => {}
                                }
                            }
                        }
                    },
                    Err(_) => {}
                }

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