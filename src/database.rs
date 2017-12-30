extern crate rusqlite;

use self::rusqlite::{Connection, Error};

use settings::Settings;
use models::channels::Channel;
use models::items::Item;

pub fn init_database(connection: &Connection, settings: &Settings) {
    /* Channel table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS channel (
        channel_id      INTEGER PRIMARY KEY,
        link            TEXT UNIQUE ON CONFLICT IGNORE,
        title           TEXT,
        description     TEXT
    )", &[]).unwrap();

    /* Item table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS item (
        item_id         INTEGER PRIMARY KEY,
        link            TEXT,
        title           TEXT,
        description     TEXT,
        is_read         BOOL,
        channel_id      INTEGER,
        FOREIGN KEY(channel_id) REFERENCES channel(channel_id),
        UNIQUE (link, channel_id) ON CONFLICT IGNORE
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

pub fn get_channels(db_connection: &Connection) -> Vec<Channel> {
    let mut channels:Vec<Channel> = vec![];

    let mut statement = db_connection.prepare("
        SELECT channel_id, link, title, description FROM channel").unwrap();
    let result_query = statement.query_map(&[], |row| {
        Channel::new(
            db_connection,
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3)
        )
    });

    match result_query {
        Ok(channels_raw) => {
            for channel in channels_raw {
                channels.push(channel.unwrap());
            }
        },
        Err(_) => {}
    }

    channels
}

pub fn update_channel(db_connection: &Connection, title: &str,
                  description: &str, channel_id: i32) -> Result<i32, rusqlite::Error>{
    db_connection.execute(
        "UPDATE channel SET title = ?, description = ? WHERE channel_id = ?",
        &[&title, &description, &channel_id]
    )
}

pub fn get_total_unread_item(db_connection: &Connection, channel_id: i32) -> i32 {
    let mut statement = db_connection.prepare("
        SELECT COUNT(item_id) FROM item WHERE item.is_read = 0 AND item.channel_id = ?").unwrap();
    let mut result = statement.query_map(&[&channel_id], |row| {
        row.get(0)
    }).unwrap();
    // TODO find a better way to write this
    result.next().unwrap().unwrap()
}

pub fn get_items_from_channel(db_connection: &Connection, channel_id: i32) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let mut statement = db_connection.prepare("
                    SELECT item_id, link, title, description, is_read FROM item WHERE channel_id = ?").unwrap();
    let result_query = statement.query_map(&[&channel_id], |row| {
        Item {
            id: row.get(0),
            link: row.get(1),
            title: row.get(2),
            description: row.get(3),
            is_read: row.get(4)
        }
    });

    match result_query {
        Ok(items_raw) => {
            for item in items_raw {
                items.push(item.unwrap());
            }
        },
        Err(_) => {}
    }

    items
}

pub fn create_item(db_connection: &Connection, link: &str, title: &str,
                   description: &str, channel_id: i32) -> Result<i32, Error> {
    db_connection.execute(
        "INSERT INTO item (link, title, description, channel_id, is_read) VALUES (?, ?, ?, ?, ?)",
        &[&link, &title, &description, &channel_id, &false]
    )
}

pub fn set_iteam_as_read(db_connection: &Connection,
                         id: i32, is_read: i32) -> Result<i32, Error> {
    db_connection.execute(
        "UPDATE item SET is_read = ? WHERE item_id = ?",
        &[&is_read, &id]
    )
}
