extern crate reqwest;
extern crate rss;
extern crate rusqlite;
extern crate atom_syndication;

use std::thread;
use std::io::Read;
use std::io::Error;
use std::str::FromStr;

use self::rusqlite::Connection;
use self::rss::Channel as RSSChannel;
use self::atom_syndication::Feed;

use components::component::Component;
use app::Cache;
use super::super::models::channels::Channel;

use database::{
    get_channels,
    create_item,
    update_channel
};

fn download_content_from_url(link: &str) -> Result<String, FetchError> {
    // Download content
    let mut content = String::new();
    reqwest::get(link)?.read_to_string(&mut content)?;
    Ok(content)
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
        let db_connection = Connection::open("base.db").unwrap();

        /* Main thread, used to launch threads to download
             * channels and parse items */
        thread::spawn(move || {
            /* List of thread to download channels */
            let mut threads_channels = vec!();

            // TODO Limit threads
            for channel in channels {
                threads_channels.push(thread::spawn(move || {
                    let channel_url = channel.link.as_ref();
                    let channel_content = download_content_from_url(channel_url);
                    (channel.id, channel_content)
                }));
            }

            // TODO non blocking
            for thread_channel in threads_channels {
                let (id, channel_content) = thread_channel.join().unwrap();
                match channel_content {
                    Ok(content) => {
                        let channel_rss_result = RSSChannel::from_str(content.as_ref());
                        match channel_rss_result {
                            Ok(channel_rss) => {

                                let result_update_channel = update_channel(
                                    &db_connection,
                                    &channel_rss.title(),
                                    &channel_rss.description(),
                                    id
                                );

                                match result_update_channel {
                                    Ok(_) => {
                                        for item in channel_rss.items() {
                                            /* Save feed in db */
                                            if let (Some(link), Some(title), Some(description)) =
                                            (item.link(), item.title(), item.description()) {
                                                let result_create_item = create_item(
                                                    &db_connection,
                                                    link,
                                                    title,
                                                    description,
                                                    id);
                                                match result_create_item {
                                                    Ok(_) => {},
                                                    Err(_) => {}
                                                }
                                            }
                                        }
                                    },
                                    Err(_) => {}
                                }
                            },
                            Err(_) => {
                                let channel_atom_result = Feed::from_str(content.as_ref());
                                match channel_atom_result {
                                    Ok(channel_atom) => {
                                        let result_update_channel = update_channel(
                                            &db_connection,
                                            &channel_atom.title(),
                                            &channel_atom.subtitle().unwrap_or(""),
                                            id
                                        );

                                        match result_update_channel {
                                            Ok(_) => {
                                                /* Save entries */
                                                for entry in channel_atom.entries() {
                                                    if let Some(content) = entry.content() {
                                                        let link_opt = entry.links().into_iter().find(|&x| x.rel() == "self");
                                                        if let (Some(link), title, Some(value)) =
                                                        (link_opt, entry.title(), content.value()) {
                                                            let result_create_item = create_item(
                                                                &db_connection,
                                                                link.href(),
                                                                title,
                                                                value,
                                                                id);
                                                            match result_create_item {
                                                                Ok(_) => {},
                                                                Err(_) => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            Err(_) => {

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
            }
            tx.send(
                format!("done")
            ).unwrap();
            match db_connection.close() {
                Ok(_) => {},
                Err(_) => {}
            }
        });
    }
}

impl Component for ComponentSync {

    fn on_init(&mut self, _cache: &Cache) {}

    fn on_key_down(&mut self, _cache: &Cache) {}

    fn on_key_up(&mut self, _cache: &Cache) {}

    fn on_key_enter(&mut self, _cache: &mut Cache) {}

    fn on_key_previous(&mut self, _cache: &mut Cache) {}

    fn on_synchronize_start(&mut self, cache: &mut Cache) {
        /* Fetch channels and save items
         * RSS and ATOM support */
        let db_connection = Connection::open("base.db").unwrap();
        let channels = get_channels(&db_connection);
        match db_connection.close() {
            Ok(_) => {},
            Err(_) => {}
        }
        self.sychronize_channels(cache, channels);
    }

    fn on_synchronize_done(&mut self, _cache: &mut Cache) {}
}
