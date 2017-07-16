extern crate rusqlite;

use self::rusqlite::Connection;

use settings::Settings;
use models::channels::{Channel, ListChannels};
use models::feeds::{Feed, ListFeeds};

pub fn init_database(connection: &Connection, settings: &Settings) {
    /* Channel table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS channel (
        channel_id INTEGER PRIMARY KEY,
        url             TEXT UNIQUE ON CONFLICT IGNORE,
        name            TEXT,
        title           TEXT
    )", &[]).unwrap();

    /* Feed table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS feed (
        feed_id         INTEGER PRIMARY KEY,
        title           TEXT,
        description     TEXT,
        is_read         BOOL,
        channel_id INTEGER,
        FOREIGN KEY(channel_id) REFERENCES channel(channel_id)
    )", &[]).unwrap();

    /* Register new channels */
    for channel in &settings.channels {
        connection.execute("
            INSERT INTO channel (url, name) VALUES (?1, ?2)",
                           &[channel, channel]).unwrap();
    }

    /* Purge old channels */
    let mut stmt = connection.prepare("SELECT url FROM channel").unwrap();
    let rows = stmt.query_map(&[], |row| -> String {row.get(0)}).unwrap();
    for row in rows {
        let url = row.unwrap();
        if !settings.channels.iter().any(|x| x == &url) {
            connection.execute("DELETE FROM channel WHERE url = ?", &[&url]).unwrap();
        }
    }
}

pub fn get_channels(db_connection: &Connection) -> ListChannels {
    let mut channels = ListChannels::new();

    let mut statement = db_connection.prepare("
        SELECT channel_id, name, url, title FROM channel").unwrap();
    let results = statement.query_map(&[], |row| {
        Channel {
            id: row.get(0),
            name: row.get(1),
            url: row.get(2),
            title: row.get(3),
        }
    }).unwrap();
    for channel in results {
        channels.add_channel(channel.unwrap());
    }
    channels
}

pub fn get_total_unread_feed(db_connection: &Connection, channel_id: i32) -> i32 {
    let mut statement = db_connection.prepare("
        SELECT COUNT(feed_id) FROM feed WHERE feed.is_read = 0 AND feed.channel_id = ?").unwrap();
    let mut results = statement.query_map(&[&channel_id], |row| {
        row.get(0)
    }).unwrap();
    // TODO beurk
    results.next().unwrap().unwrap()
}

pub fn get_feeds_from_channel(db_connection: &Connection, channel_id: i32) -> ListFeeds {
    let mut feeds = ListFeeds::new();
    let mut statement = db_connection.prepare("
                    SELECT title, description, is_read FROM feed WHERE channel_id = ?").unwrap();
    let rows = statement.query_map(&[&channel_id], |row| {
        Feed {
            title: row.get(0),
            description: row.get(1),
            is_read: row.get(2)
        }
    }).unwrap();
    for row in rows {
        feeds.add_feed(row.unwrap());
    }
    feeds
}

pub fn create_feed(db_connection: &Connection, title: &str,
                   description: &str, channel_id: i32) {
    db_connection.execute(
        "INSERT INTO feed (title, description, channel_id, is_read) VALUES (?, ?, ?, ?)",
        &[&title, &description, &channel_id, &false]
    ).unwrap();
}