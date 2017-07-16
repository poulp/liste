extern crate rusqlite;

use self::rusqlite::Connection;

use settings::Settings;
use models::channels::{Channel, ListChannels};
use models::items::{Item, ListItems};

pub fn init_database(connection: &Connection, settings: &Settings) {
    /* Channel table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS channel (
        channel_id      INTEGER PRIMARY KEY,
        link            TEXT UNIQUE ON CONFLICT IGNORE,
        title           TEXT
    )", &[]).unwrap();

    /* Item table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS item (
        item_id         INTEGER PRIMARY KEY,
        title           TEXT,
        description     TEXT,
        is_read         BOOL,
        channel_id      INTEGER,
        FOREIGN KEY(channel_id) REFERENCES channel(channel_id)
    )", &[]).unwrap();

    /* Register new channels */
    for channel in &settings.channels {
        connection.execute("
            INSERT INTO channel (link) VALUES (?1)",
                           &[channel]).unwrap();
    }

    /* Purge old channels */
    let mut stmt = connection.prepare("SELECT link FROM channel").unwrap();
    let rows = stmt.query_map(&[], |row| -> String {row.get(0)}).unwrap();
    for row in rows {
        let link = row.unwrap();
        if !settings.channels.iter().any(|x| x == &link) {
            connection.execute("DELETE FROM channel WHERE link = ?", &[&link]).unwrap();
        }
    }
}

pub fn get_channels(db_connection: &Connection) -> ListChannels {
    let mut channels = ListChannels::new();

    let mut statement = db_connection.prepare("
        SELECT channel_id, link, title FROM channel").unwrap();
    let results = statement.query_map(&[], |row| {
        Channel {
            id: row.get(0),
            link: row.get(1),
            title: row.get(2),
        }
    }).unwrap();
    for channel in results {
        channels.add_channel(channel.unwrap());
    }
    channels
}

pub fn get_total_unread_item(db_connection: &Connection, channel_id: i32) -> i32 {
    let mut statement = db_connection.prepare("
        SELECT COUNT(item_id) FROM item WHERE item.is_read = 0 AND item.channel_id = ?").unwrap();
    let mut results = statement.query_map(&[&channel_id], |row| {
        row.get(0)
    }).unwrap();
    // TODO beurk
    results.next().unwrap().unwrap()
}

pub fn get_items_from_channel(db_connection: &Connection, channel_id: i32) -> ListItems {
    let mut items = ListItems::new();
    let mut statement = db_connection.prepare("
                    SELECT title, description, is_read FROM item WHERE channel_id = ?").unwrap();
    let rows = statement.query_map(&[&channel_id], |row| {
        Item {
            title: row.get(0),
            description: row.get(1),
            is_read: row.get(2)
        }
    }).unwrap();
    for row in rows {
        items.add_item(row.unwrap());
    }
    items
}

pub fn create_item(db_connection: &Connection, title: &str,
                   description: &str, channel_id: i32) {
    db_connection.execute(
        "INSERT INTO item (title, description, channel_id, is_read) VALUES (?, ?, ?, ?)",
        &[&title, &description, &channel_id, &false]
    ).unwrap();
}